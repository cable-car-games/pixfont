// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use glam::Vec2;
use iced::{
    Element, Task,
    widget::{Button, Column, Container, Row, Scrollable, Text},
};

use crate::ui::widgets::glyph_editor::GlyphEditor;

pub struct Editor {
    scale: f32,
    offset: Vec2,
}

#[derive(Debug, Clone)]
pub enum Message {
    Private(PrivateMessage),
}

#[derive(Debug, Clone)]
pub enum PrivateMessage {
    SetScale(f32),
    SetOffset(Vec2),
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            scale: 16.0,
            offset: Vec2::ZERO,
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
            .push(Row::new().push(Button::new("Do shit")))
            .spacing(2);

        Row::new()
            .push(Scrollable::new(inspector))
            .push(
                Container::new(
                    Column::new().push(toolbar).push(
                        GlyphEditor::new(glyph)
                            .scale(self.scale)
                            .offset(self.offset)
                            .on_scale(|scale| Message::Private(PrivateMessage::SetScale(scale)))
                            .on_pan(|offset| Message::Private(PrivateMessage::SetOffset(offset))),
                    ),
                )
                .style(iced::widget::container::bordered_box),
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
        }
    }
}
