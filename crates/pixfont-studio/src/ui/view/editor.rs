// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use iced::{
    Element, Length, Task, Vector,
    widget::{Button, Column, Container, Row, Scrollable, Space, Text, button, text, text_input},
};
use iced_aw::number_input;

use crate::{
    settings::Settings,
    ui::widgets::{
        glyph_editor::{GlyphEditor, Tool},
        icon::Icon,
        inspector,
    },
};

pub struct Editor {
    scale: f32,
    offset: Vector<f32>,
    tool: Tool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Private(PrivateMessage),
    SetGlyphProp(GlyphProp),
    SetGuideline(GuidelineAction),
}

#[derive(Debug, Clone)]
pub enum GlyphProp {
    Advance(u32),
    Ascender(i32),
    Descender(i32),
    CapHeight(i32),
    XHeight(i32),
}

#[derive(Debug, Clone)]
pub enum GuidelineAction {
    /// Create a new empty guideline.
    Create,

    /// Rename a guideline.
    Rename {
        scope: GuidelineScope,
        direction: GuidelineDirection,
        index: usize,
        name: String,
    },

    /// Set a guideline.
    Set {
        scope: GuidelineScope,
        direction: GuidelineDirection,
        index: usize,
        position: i32,
    },

    /// Remove a guideline.
    Remove {
        scope: GuidelineScope,
        direction: GuidelineDirection,
        index: usize,
    },

    /// Move a local (glyph-specific) guideline to the global scope (font-wide).
    MakeGlobal {
        direction: GuidelineDirection,
        index: usize,
    },

    /// Move a global (font-wide) guideline to the local scope.
    MakeLocal {
        direction: GuidelineDirection,
        index: usize,
    },

    /// Make a vertical guideline horizontal.
    MakeX { scope: GuidelineScope, index: usize },

    /// Make a horizontal guideline vertical.
    MakeY { scope: GuidelineScope, index: usize },
}

#[derive(Debug, Clone, Copy)]
pub enum GuidelineScope {
    Global,
    Local,
}

#[derive(Debug, Clone, Copy)]
pub enum GuidelineDirection {
    X,
    Y,
}

#[derive(Debug, Clone)]
pub enum PrivateMessage {
    SetScale(f32),
    SetOffset(Vector<f32>),
    SetTool(Tool),
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            scale: 16.0,
            offset: Default::default(),
            tool: Tool::Pen,
        }
    }
}

impl Editor {
    pub fn view<'state>(
        &'state self,
        settings: &'state Settings,
        font: &'state pixfont::Font,
        selected_glyph_name: Option<&'state String>,
    ) -> Element<'state, Message> {
        let Some(selected_glyph_name) = selected_glyph_name else {
            // TODO: disable the tab while a glyph is not selected, or select a default gltph
            return Text::new("(no glyph selected)").into();
        };

        let Some(glyph) = font.glyphs.get(selected_glyph_name) else {
            // this should ideally never happen, but we check for it anyway
            return Text::new("(glyph does not exist)").into();
        };

        let inspector = Column::new()
            .width(320)
            .spacing(8)
            .push(
                inspector::section("Glyph")
                    .push(inspector::property("Glyph name", text(selected_glyph_name)))
                    .push(inspector::property("Codepoint", text("U+...")))
                    .push(inspector::property("Alternate", text("...")))
                    .push(inspector::property(
                        "Advance",
                        number_input(&glyph.advance, 0..(i32::MAX as u32), |value| {
                            Message::SetGlyphProp(GlyphProp::Advance(value))
                        }),
                    )),
            )
            .push(
                inspector::section("Metrics")
                    .push(inspector::property(
                        "Em height",
                        text((font.metrics.ascender - font.metrics.descender).to_string()),
                    ))
                    .push(inspector::property(
                        "Ascender",
                        number_input(&font.metrics.ascender, 0..i32::MAX, |value| {
                            Message::SetGlyphProp(GlyphProp::Ascender(value))
                        })
                        .width(Length::Fill),
                    ))
                    .push(inspector::property(
                        "Descender",
                        number_input(&font.metrics.descender, i32::MIN..0, |value| {
                            Message::SetGlyphProp(GlyphProp::Descender(value))
                        })
                        .width(Length::Fill),
                    ))
                    .push(inspector::property(
                        "Cap height",
                        number_input(&font.metrics.cap_height, 0..i32::MAX, |value| {
                            Message::SetGlyphProp(GlyphProp::CapHeight(value))
                        })
                        .width(Length::Fill),
                    ))
                    .push(inspector::property(
                        "x height",
                        number_input(&font.metrics.x_height, 0..i32::MAX, |value| {
                            Message::SetGlyphProp(GlyphProp::XHeight(value))
                        })
                        .width(Length::Fill),
                    )),
            )
            .push(
                inspector::section("Guidelines")
                    .extend(
                        [
                            (
                                GuidelineScope::Global,
                                GuidelineDirection::X,
                                &font.metrics.guidelines.x,
                            ),
                            (
                                GuidelineScope::Global,
                                GuidelineDirection::Y,
                                &font.metrics.guidelines.y,
                            ),
                            (
                                GuidelineScope::Local,
                                GuidelineDirection::X,
                                &glyph.guidelines.x,
                            ),
                            (
                                GuidelineScope::Local,
                                GuidelineDirection::Y,
                                &glyph.guidelines.y,
                            ),
                        ]
                        .into_iter()
                        .flat_map(|(scope, direction, vec)| {
                            vec.iter().enumerate().map(move |(index, guideline)| {
                                Row::new()
                                    .spacing(2)
                                    .push(text_input("(key)", &guideline.name).on_input(
                                        move |name| {
                                            Message::SetGuideline(GuidelineAction::Rename {
                                                scope,
                                                direction,
                                                index,
                                                name,
                                            })
                                        },
                                    ))
                                    .push(number_input(
                                        &guideline.position,
                                        i32::MIN..i32::MAX,
                                        move |value| {
                                            Message::SetGuideline(GuidelineAction::Set {
                                                scope,
                                                direction,
                                                index,
                                                position: value,
                                            })
                                        },
                                    ))
                                    .push(
                                        button(match direction {
                                            GuidelineDirection::X => "X",
                                            GuidelineDirection::Y => "Y",
                                        })
                                        .style(iced::widget::button::subtle)
                                        .on_press(
                                            Message::SetGuideline(match direction {
                                                GuidelineDirection::X => {
                                                    GuidelineAction::MakeY { scope, index }
                                                }
                                                GuidelineDirection::Y => {
                                                    GuidelineAction::MakeX { scope, index }
                                                }
                                            }),
                                        ),
                                    )
                                    .push(
                                        button(match scope {
                                            GuidelineScope::Global => "G",
                                            GuidelineScope::Local => "g",
                                        })
                                        .on_press(
                                            Message::SetGuideline(match scope {
                                                GuidelineScope::Global => {
                                                    GuidelineAction::MakeLocal { direction, index }
                                                }
                                                GuidelineScope::Local => {
                                                    GuidelineAction::MakeGlobal { direction, index }
                                                }
                                            }),
                                        ),
                                    )
                                    .push(button("×").style(iced::widget::button::danger).on_press(
                                        Message::SetGuideline(GuidelineAction::Remove {
                                            scope,
                                            direction,
                                            index,
                                        }),
                                    ))
                                    .into()
                            })
                        }),
                    )
                    .push(
                        button("Add guideline")
                            .style(iced::widget::button::subtle)
                            .on_press(Message::SetGuideline(GuidelineAction::Create)),
                    ),
            );

        let toolbar = Row::new()
            .push(
                Row::new()
                    .push(
                        Button::new(Icon::BiArrowCounterclockwise.as_svg())
                            .style(iced::widget::button::subtle),
                    )
                    .push(
                        Button::new(Icon::BiArrowClockwise.as_svg())
                            .style(iced::widget::button::subtle),
                    )
                    .push(Space::new())
                    .push(Button::new(Icon::BiCopy.as_svg()).style(iced::widget::button::subtle))
                    .push(
                        Button::new(Icon::BiClipboard.as_svg()).style(iced::widget::button::subtle),
                    )
                    .spacing(4)
                    .width(Length::Fill),
            )
            .push(
                Row::new()
                    .extend(
                        [
                            (Tool::Pen, Icon::BiPen, "Pen"),
                            (Tool::Line, Icon::BiSlashLg, "Line"),
                            (Tool::Rectangle, Icon::BiSquare, "Rectangle"),
                            (Tool::Fill, Icon::BiPaintBucket, "Fill"),
                            (Tool::Eraser, Icon::BiEraser, "Eraser"),
                            (Tool::Pan, Icon::BiArrowsMove, "Pan"),
                        ]
                        .map(|(tool, icon, _name)| {
                            Button::new(icon.as_svg())
                                .on_press(Message::Private(PrivateMessage::SetTool(tool)))
                                .style(if tool == self.tool {
                                    iced::widget::button::primary
                                } else {
                                    iced::widget::button::subtle
                                })
                                .into()
                        }),
                    )
                    .spacing(4),
            )
            .push(
                Container::new(
                    Row::new()
                        .push(
                            Button::new(Icon::BiBorderInner.as_svg())
                                .style(iced::widget::button::subtle),
                        )
                        .push(
                            Button::new(Icon::BiZoomIn.as_svg())
                                .style(iced::widget::button::subtle),
                        )
                        .push(
                            Button::new(Icon::BiZoomOut.as_svg())
                                .style(iced::widget::button::subtle),
                        )
                        .spacing(4),
                )
                .align_right(Length::Fill),
            )
            .spacing(2);

        Row::new()
            .spacing(8)
            .push(Scrollable::new(inspector))
            .push(
                Container::new(
                    Column::new()
                        .push(toolbar)
                        .push(
                            GlyphEditor::new(glyph, &font.metrics)
                                .scale(self.scale)
                                .offset(self.offset)
                                .guidelines(font.metrics.guidelines.clone())
                                .guidelines(glyph.guidelines.clone())
                                .tool(self.tool)
                                .colors(settings.appearance.editor)
                                .on_scale(|scale| Message::Private(PrivateMessage::SetScale(scale)))
                                .on_pan(|offset| {
                                    Message::Private(PrivateMessage::SetOffset(offset))
                                }),
                        )
                        .spacing(4),
                )
                .style(iced::widget::container::bordered_box)
                .padding(4),
            )
            .into()
    }

    pub fn update(&mut self, message: PrivateMessage) -> Task<Message> {
        match message {
            PrivateMessage::SetScale(scale) => {
                self.scale = scale;
                Task::none()
            }
            PrivateMessage::SetOffset(offset) => {
                self.offset = offset;
                Task::none()
            }
            PrivateMessage::SetTool(tool) => {
                self.tool = tool;
                Task::none()
            }
        }
    }
}
