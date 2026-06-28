// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use iced::{
    Element, Length, Task, Vector,
    widget::{Button, Column, Container, Row, Scrollable, Space, Text},
};

use crate::ui::widgets::{
    glyph_editor::{GlyphEditor, Tool},
    icon::Icon,
};

pub struct Editor {
    scale: f32,
    offset: Vector<f32>,
    tool: Tool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Private(PrivateMessage),
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
        _font: &'state pixfont::Font,
        selected_glyph_name: &Option<String>,
    ) -> Element<'state, Message> {
        let Some(selected_glyph_name) = selected_glyph_name else {
            // TODO: disable the tab while a glyph is not selected, or select a default gltph
            return Text::new("(no glyph selected)").into();
        };

        let Some(glyph) = _font.glyphs.get(selected_glyph_name) else {
            // this should ideally never happen, but we check for it anyway
            return Text::new("(glyph does not exist)").into();
        };

        let inspector = Column::new().width(320).spacing(8);

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
            .push(Scrollable::new(inspector))
            .push(
                Container::new(
                    Column::new()
                        .push(toolbar)
                        .push(
                            GlyphEditor::new(glyph)
                                .scale(self.scale)
                                .offset(self.offset)
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

    pub fn update<'state>(&'state mut self, message: PrivateMessage) -> Task<Message> {
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
