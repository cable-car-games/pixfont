// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use iced::{
    Element, Length,
    widget::{Button, Column, Container, Row, Text},
};
use iced_aw::{DropDown, drop_down::Alignment};

use crate::ui::widgets::icon::Icon;

pub struct Topbar {
    pub view: View,
    pub export_shown: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    NewFile,
    OpenFile,
    SaveFile,
    ShowView(View),

    OpenExportDropdown(Option<bool>),
    ExportFile(pixfont::export::Exporter),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum View {
    Glyphs,
    Edit,
    Settings,
}

impl Default for Topbar {
    fn default() -> Self {
        Self {
            view: View::Glyphs,
            export_shown: Default::default(),
        }
    }
}

impl Topbar {
    pub fn view(&self) -> Element<'_, Message> {
        let button_style = |view: View| {
            if view == self.view {
                iced::widget::button::primary
            } else {
                iced::widget::button::subtle
            }
        };

        Row::new()
            .push(
                Container::new(
                    Row::new()
                        .push(
                            Button::new(Icon::BiPlusLg.as_svg())
                                .style(iced::widget::button::subtle)
                                .on_press(Message::NewFile),
                        )
                        .push(
                            Button::new(Icon::BiFolder2Open.as_svg())
                                .style(iced::widget::button::subtle)
                                .on_press(Message::OpenFile),
                        )
                        .push(
                            Button::new(Icon::BiFloppy.as_svg())
                                .style(iced::widget::button::subtle)
                                .on_press(Message::SaveFile),
                        )
                        .push(
                            DropDown::new(
                                Button::new(Icon::BiFileArrowDown.as_svg())
                                    .style(if self.export_shown {
                                        iced::widget::button::primary
                                    } else {
                                        iced::widget::button::subtle
                                    })
                                    .on_press(Message::OpenExportDropdown(None)),
                                Container::new(Column::with_children(
                                    pixfont::export::EXPORTERS.iter().map(|exporter| {
                                        Button::new(Text::new(format!("{}", exporter)))
                                            .style(iced::widget::button::text)
                                            .width(240)
                                            .on_press(Message::ExportFile(*exporter))
                                            .into()
                                    }),
                                ))
                                .style(iced::widget::container::bordered_box),
                                self.export_shown,
                            )
                            .width(Length::Fill)
                            .alignment(Alignment::Bottom)
                            .on_dismiss(Message::OpenExportDropdown(Some(false))),
                        )
                        .spacing(4),
                )
                .align_left(Length::Fill),
            )
            .push(
                Row::new()
                    .push(
                        Button::new("Glyphs")
                            .style(button_style(View::Glyphs))
                            .on_press(Message::ShowView(View::Glyphs)),
                    )
                    .push(
                        Button::new("Edit")
                            .style(button_style(View::Edit))
                            .on_press(Message::ShowView(View::Edit)),
                    )
                    .spacing(4),
            )
            .push(
                Container::new(
                    Row::new()
                        .push(
                            Button::new(Icon::BiGearWideConnected.as_svg())
                                .style(button_style(View::Settings))
                                .on_press(Message::ShowView(View::Settings)),
                        )
                        .spacing(4),
                )
                .align_right(Length::Fill),
            )
            .spacing(4)
            .into()
    }
}
