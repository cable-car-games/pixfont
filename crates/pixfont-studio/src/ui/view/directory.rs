// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use std::{fmt::Display, ops::Deref};

use iced::{
    Element, Length, Task,
    widget::{Button, Column, Container, Grid, PickList, Row, Scrollable, Text, TextInput},
};
use iced_aw::{DropDown, drop_down::Alignment};
use ucd::Block;

use crate::ui::{
    view::directory::new_from_unicode::NewFromUnicode,
    widgets::{icon::Icon, inspector},
};

mod new_from_unicode;

#[derive(Default)]
pub struct Directory {
    filter: Option<String>,
    order: DirectoryOrder,

    set_dropdown_shown: bool,
    new_from_unicode: NewFromUnicode,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DirectoryOrder {
    /// Don't order
    None,

    /// Order by glyph name
    Name,

    /// Order by unicode mapping
    #[default]
    Unicode,
}

impl Display for DirectoryOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            DirectoryOrder::None => "None",
            DirectoryOrder::Name => "Name",
            DirectoryOrder::Unicode => "Unicode",
        })
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectGlyph(String),

    SetFilter(String),
    SetOrder(DirectoryOrder),

    SetDropdown(Option<bool>),

    // metadata update messages
    SetName(String),
    SetFamily(Option<String>),
    SetWeight(Option<String>),
    SetStyle(Option<String>),
    SetAuthor(Option<String>),
    SetCopyright(Option<String>),
    SetLicense(Option<String>),
    AddNewExtra,
    SetExtraKey(String, String),
    SetExtraValue(String, String),
    RemoveExtra(String),

    // glyph management
    AddGlyphsFromSet(pixfont::sets::GlyphSet),
    AddGlyphsFromUnicodeBlock,

    // unicode thing
    CloseAddNewBlocks,
    SelectBlock(Block),
    SubmitBlock(Block),
}

impl Directory {
    fn glyphs<'state>(
        &'state self,
        font: &'state pixfont::Font,
        selected_glyph: &'state Option<String>,
    ) -> Element<'state, Message> {
        let glyphs: Vec<(&'state String, &'state pixfont::Glyph)> = match self.order {
            DirectoryOrder::None => font.glyphs.iter().collect(),
            DirectoryOrder::Name => {
                let mut pairs: Vec<_> = font.glyphs.iter().collect();
                pairs.sort_by_key(|(name, _)| *name);
                pairs
            }
            DirectoryOrder::Unicode => {
                let mut codepoints: Vec<u32> = font.mappings.keys().copied().collect();
                codepoints.sort();
                codepoints
                    .iter()
                    .flat_map(|codepoint| {
                        let mapping = &font.mappings[codepoint];
                        let mut glyph_names = vec![&mapping.glyph];
                        glyph_names.extend(mapping.alternate.values());
                        glyph_names
                            .iter()
                            .flat_map(|glyph_name| {
                                let Some(glyph) = font.glyphs.get(*glyph_name) else {
                                    println!("glyph does not exist! {}", glyph_name);
                                    return None;
                                };

                                Some((*glyph_name, glyph))
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect()
            }
        };

        Container::new(
            Grid::with_children(glyphs.iter().map(|(name, _glyph)| {
                let name = *name;
                let _glyph = *_glyph;

                // TODO: glyph preview and mappings
                Button::new(name.deref())
                    .style(
                        if let Some(selected_glyph) = selected_glyph
                            && *selected_glyph == **name
                        {
                            iced::widget::button::primary
                        } else {
                            iced::widget::button::background
                        },
                    )
                    .on_press(Message::SelectGlyph(name.to_string()))
                    .into()
            }))
            .fluid(120)
            .spacing(4),
        )
        .into()
    }

    pub fn view<'state>(
        &'state self,
        font: &'state pixfont::Font,
        selected_glyph: &'state Option<String>,
    ) -> Element<'state, Message> {
        let orders = [
            DirectoryOrder::None,
            DirectoryOrder::Name,
            DirectoryOrder::Unicode,
        ];

        let inspector = Column::new()
            .push(
                inspector::section("Metadata")
                    .push(inspector::property(
                        "Name",
                        TextInput::new("Pixel Sans", &font.metadata.name)
                            .on_input(Message::SetName),
                    ))
                    .push(inspector::property(
                        "Family",
                        TextInput::new("Pixel Sans", font.metadata.family.as_deref().unwrap_or(""))
                            .on_input(|s| {
                                Message::SetFamily(if s.is_empty() { None } else { Some(s) })
                            }),
                    ))
                    .push(inspector::property(
                        "Weight",
                        TextInput::new("Regular", font.metadata.weight.as_deref().unwrap_or(""))
                            .on_input(|s| {
                                Message::SetWeight(if s.is_empty() { None } else { Some(s) })
                            }),
                    ))
                    .push(inspector::property(
                        "Style",
                        TextInput::new("Roman", font.metadata.style.as_deref().unwrap_or(""))
                            .on_input(|s| {
                                Message::SetStyle(if s.is_empty() { None } else { Some(s) })
                            }),
                    ))
                    .push(inspector::property(
                        "Author",
                        TextInput::new("Jane Doe", font.metadata.author.as_deref().unwrap_or(""))
                            .on_input(|s| {
                                Message::SetAuthor(if s.is_empty() { None } else { Some(s) })
                            }),
                    ))
                    .push(inspector::property(
                        "Copyright",
                        TextInput::new(
                            "2026 Jane Doe",
                            font.metadata.copyright.as_deref().unwrap_or(""),
                        )
                        .on_input(|s| {
                            Message::SetCopyright(if s.is_empty() { None } else { Some(s) })
                        }),
                    ))
                    .push(inspector::property(
                        "Licence",
                        TextInput::new(
                            "SIL OFL 1.1",
                            font.metadata.license.as_deref().unwrap_or(""),
                        )
                        .on_input(|s| {
                            Message::SetLicense(if s.is_empty() { None } else { Some(s) })
                        }),
                    )),
            )
            .push(
                inspector::section("Extra")
                    .push(
                        Column::from_iter(font.metadata.extra.iter().map(|(key, value)| {
                            let old_key = key.clone();
                            Row::new()
                                .spacing(2)
                                .push(
                                    TextInput::new("(key)", &old_key)
                                        .width(inspector::LABEL_WIDTH)
                                        .on_input(move |value| {
                                            Message::SetExtraKey(old_key.clone(), value)
                                        }),
                                )
                                .push(TextInput::new("(value)", value).on_input(|value| {
                                    Message::SetExtraValue(key.to_string(), value)
                                }))
                                .push(
                                    Button::new("×")
                                        .style(iced::widget::button::danger)
                                        .on_press(Message::RemoveExtra(key.to_string())),
                                )
                                .into()
                        }))
                        .spacing(2),
                    )
                    .push(
                        Button::new("Add extra data")
                            .style(iced::widget::button::subtle)
                            .on_press(Message::AddNewExtra),
                    ),
            )
            .push(inspector::section("Glyph").push(inspector::property(
                "Glyph name",
                TextInput::new("(not selected)", selected_glyph.as_deref().unwrap_or("")),
            )))
            .width(320)
            .spacing(8);

        let new_set_dropdown = DropDown::new(
            Button::new(
                Row::new()
                    .push(Icon::BiPlusLg.as_svg())
                    .push("New from set")
                    .spacing(4),
            )
            .style(if self.set_dropdown_shown {
                iced::widget::button::primary
            } else {
                iced::widget::button::subtle
            })
            .on_press(Message::SetDropdown(None)),
            Container::new(
                Column::with_children(pixfont::sets::DEFINED_GLYPH_SETS.iter().map(|set| {
                    Button::new(Text::new(format!("{}", set)))
                        .style(iced::widget::button::text)
                        .width(240)
                        .on_press(Message::AddGlyphsFromSet(*set))
                        .into()
                }))
                .push(
                    Button::new("Unicode block...")
                        .style(iced::widget::button::text)
                        .width(240)
                        .on_press(Message::AddGlyphsFromUnicodeBlock),
                ),
            )
            .style(iced::widget::container::bordered_box),
            self.set_dropdown_shown,
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .alignment(Alignment::Bottom)
        .on_dismiss(Message::SetDropdown(Some(false)));

        let toolbar = Row::new()
            .push(
                Row::new()
                    //.push(
                    //    Button::new(
                    //        Row::new()
                    //            .push(Icon::BiPlusLg.as_svg())
                    //            .push("New glyph")
                    //            .spacing(4),
                    //    )
                    //    .style(iced::widget::button::subtle)
                    //    .on_press(Message::Noop),
                    //)
                    .push(new_set_dropdown)
                    .spacing(2),
            )
            .push(
                Container::new(
                    Row::new()
                        .push(
                            TextInput::new(
                                "Search",
                                self.filter.clone().unwrap_or_default().as_str(),
                            )
                            .on_input(Message::SetFilter)
                            .width(140),
                        )
                        .push(PickList::new(orders, Some(self.order), |order| {
                            Message::SetOrder(order)
                        }))
                        .spacing(4),
                )
                .align_right(Length::Fill),
            );

        let directory = self.glyphs(font, selected_glyph);

        Row::new()
            .push(Scrollable::new(inspector))
            .push(
                self.new_from_unicode.view(
                    Container::new(
                        Column::new()
                            .push(toolbar)
                            .push(
                                Scrollable::new(directory)
                                    .width(Length::Fill)
                                    .height(Length::Fill)
                                    .spacing(4),
                            )
                            .spacing(4),
                    )
                    .padding(4)
                    .style(iced::widget::container::bordered_box),
                    Message::SelectBlock,
                    Message::SubmitBlock,
                    Message::CloseAddNewBlocks,
                ),
            )
            .spacing(8)
            .into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SetFilter(str) => {
                self.filter = if str.is_empty() {
                    None
                } else {
                    Some(str.clone())
                };
                Task::none()
            }
            Message::SetOrder(order) => {
                self.order = order;
                Task::none()
            }
            Message::SetDropdown(show) => {
                self.set_dropdown_shown = show.unwrap_or(!self.set_dropdown_shown);
                Task::none()
            }
            Message::AddGlyphsFromUnicodeBlock => {
                self.set_dropdown_shown = false;
                self.new_from_unicode.show = true;
                Task::none()
            }
            Message::CloseAddNewBlocks => {
                self.new_from_unicode.show = false;
                Task::none()
            }
            Message::SelectBlock(block) => {
                self.new_from_unicode.selected = block;
                Task::none()
            }
            Message::SubmitBlock(_) => {
                self.new_from_unicode.show = false;
                Task::none()
            }
            _ => todo!(),
        }
    }
}
