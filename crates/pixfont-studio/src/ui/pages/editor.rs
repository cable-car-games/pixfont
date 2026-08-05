// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use std::{collections::LinkedList, ops::Not};

use iced::{
    Element, Length, Task, Vector,
    alignment::Vertical,
    widget::{Button, Column, Container, Row, Space, button, container, text},
};
use pixfont::Glyph;
use pixicons::icon::icon;

use crate::{
    settings::Settings,
    ui::widgets::{
        glyph_editor::{Delta, GlyphEditor, Tool},
        tooltip::toolbar_tooltip,
    },
};

mod font_inspector;

pub struct Editor {
    glyph_name: Option<String>,

    scale: f32,
    offset: Vector<f32>,
    tool: Tool,

    undo_stack: LinkedList<Delta>,
    redo_stack: LinkedList<Delta>,

    inspector: font_inspector::State,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Message {
    SelectGlyph(Option<String>),

    // old...
    Apply(Delta),
    SetScale(f32),
    SetOffset(Vector<f32>),
    SetTool(Tool),
    Undo,
    Redo,
    Copy,
    Paste,
    ResetViewport,
    ZoomIn,
    ZoomOut,
    Inspector(font_inspector::Message),
}

impl From<Message> for crate::Message {
    fn from(message: Message) -> Self {
        Self::Editor(message)
    }
}

const SCALE_MIN: f32 = 2.0;
const SCALE_MAX: f32 = 128.0;
const DEFAULT_SCALE: f32 = 16.0;

impl Default for Editor {
    fn default() -> Self {
        Self {
            glyph_name: None,
            scale: DEFAULT_SCALE,
            offset: Default::default(),
            tool: Tool::Pen,
            undo_stack: LinkedList::new(),
            redo_stack: LinkedList::new(),
            inspector: Default::default(),
        }
    }
}

impl Editor {
    pub fn view<'state>(
        &'state self,
        settings: &'state Settings,
        font: &'state pixfont::Font,
    ) -> Element<'state, crate::Message> {
        let toolbar: Row<'_, crate::Message> = Row::new()
            .push(
                Row::new()
                    .push(toolbar_tooltip(
                        button(icon!(undo))
                            .style(iced::widget::button::subtle)
                            .on_press_maybe(
                                self.undo_stack
                                    .is_empty()
                                    .not()
                                    .then_some(Message::Undo.into()),
                            ),
                        "Undo",
                    ))
                    .push(toolbar_tooltip(
                        Button::new(icon!(redo))
                            .style(iced::widget::button::subtle)
                            .on_press_maybe(
                                self.redo_stack
                                    .is_empty()
                                    .not()
                                    .then_some(Message::Redo.into()),
                            ),
                        "Redo",
                    ))
                    .push(Space::new())
                    //.push(
                    //    Button::new(Icon::BiCopy.as_svg())
                    //        .style(iced::widget::button::subtle)
                    //        .on_press(Message::Copy),
                    //)
                    //.push(
                    //    Button::new(Icon::BiClipboard.as_svg())
                    //        .style(iced::widget::button::subtle)
                    //        .on_press(Message::Paste),
                    //)
                    .spacing(4)
                    .width(Length::Fill),
            )
            .push(
                Row::new()
                    .extend(
                        [
                            (Tool::Pen, icon!(pen), "Pen"),
                            (Tool::Line, icon!(line), "Line"),
                            (Tool::Rectangle, icon!(rectangle), "Rectangle"),
                            //(Tool::Fill, icon!(fill), "Fill"),
                            (Tool::Eraser, icon!(eraser), "Eraser"),
                            (Tool::Pan, icon!(grab), "Pan"),
                        ]
                        .map(|(tool, icon, name)| {
                            toolbar_tooltip(
                                button(icon).on_press(Message::SetTool(tool).into()).style(
                                    if tool == self.tool {
                                        iced::widget::button::primary
                                    } else {
                                        iced::widget::button::subtle
                                    },
                                ),
                                name,
                            )
                            .into()
                        }),
                    )
                    .spacing(4),
            )
            .push(
                Container::new(
                    Row::new()
                        .align_y(Vertical::Center)
                        .push(toolbar_tooltip(
                            button(icon!(zoom.zero))
                                .style(iced::widget::button::subtle)
                                .on_press(Message::ResetViewport.into()),
                            "Reset viewport",
                        ))
                        .push(toolbar_tooltip(
                            Button::new(icon!(zoom.out))
                                .style(iced::widget::button::subtle)
                                .on_press(Message::ZoomOut.into()),
                            "Zoom out",
                        ))
                        .push(toolbar_tooltip(
                            button(text(format!("{:.0}%", self.scale * 100.0)))
                                .style(button::subtle)
                                .on_press(Message::SetScale(DEFAULT_SCALE).into()),
                            "Reset zoom",
                        ))
                        .push(toolbar_tooltip(
                            Button::new(icon!(zoom.in))
                                .style(iced::widget::button::subtle)
                                .on_press(Message::ZoomIn.into()),
                            "Zoom in",
                        ))
                        .spacing(4),
                )
                .align_right(Length::Fill),
            )
            .spacing(2);

        let editor: Container<'_, crate::Message> = if let Some(glyph) = &self.glyph_name {
            if let Some(glyph) = font.glyphs.get(glyph) {
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
                                .on_scale(|value| Message::SetScale(value).into())
                                .on_pan(|value| Message::SetOffset(value).into())
                                .on_tool(|value| Message::SetTool(value).into())
                                .on_apply(|value| Message::Apply(value).into()),
                        )
                        .spacing(4),
                )
                .style(iced::widget::container::bordered_box)
                .padding(4)
            } else {
                no_glyph()
            }
        } else {
            no_glyph()
        };

        Row::new()
            .spacing(8)
            .push(self.inspector.view(font, &self.glyph_name))
            .push(editor)
            .into()
    }

    pub fn update(&mut self, message: Message, font: &mut pixfont::Font) -> Task<crate::Message> {
        match message {
            Message::SelectGlyph(glyph_name) => {
                self.glyph_name = glyph_name;
                Task::none()
            }

            Message::Apply(delta) => {
                let glyph = self.glyph_name.as_ref().expect("bug: no glyph selected");
                let glyph = font
                    .glyphs
                    .get_mut(glyph)
                    .expect("bug: glyph does not exist");

                // TODO: filter out points already on the glyph
                self.undo_stack.push_back(delta.clone());
                apply(glyph, &delta);
                Task::none()
            }

            Message::SetScale(scale) => {
                self.set_scale(scale);
                Task::none()
            }

            Message::SetOffset(offset) => {
                self.offset = offset;
                Task::none()
            }

            Message::SetTool(tool) => {
                self.tool = tool;
                Task::none()
            }

            Message::Undo => {
                let glyph = self.glyph_name.as_ref().expect("bug: no glyph selected");
                let glyph = font
                    .glyphs
                    .get_mut(glyph)
                    .expect("bug: glyph does not exist");

                let Some(delta) = self.undo_stack.pop_back() else {
                    println!("empty undo stack");
                    return Task::none();
                };
                self.redo_stack.push_back(delta.clone());

                apply(
                    glyph,
                    &Delta {
                        add: delta.remove,
                        remove: delta.add,
                    },
                );

                Task::none()
            }

            Message::Redo => {
                let glyph = self.glyph_name.as_ref().expect("bug: no glyph selected");
                let glyph = font
                    .glyphs
                    .get_mut(glyph)
                    .expect("bug: glyph does not exist");

                let Some(delta) = self.redo_stack.pop_back() else {
                    println!("empty redo stack");
                    return Task::none();
                };

                apply(glyph, &delta);
                self.undo_stack.push_back(delta);

                Task::none()
            }

            Message::Copy => todo!(),

            Message::Paste => todo!(),

            Message::ResetViewport => {
                self.offset = Default::default();
                self.scale = DEFAULT_SCALE;
                Task::none()
            }

            Message::ZoomIn => {
                self.set_scale(self.scale * 2.0);
                Task::none()
            }

            Message::ZoomOut => {
                self.set_scale(self.scale / 2.0);
                Task::none()
            }
            Message::Inspector(message) => self.inspector.update(font, message),
        }
    }

    fn set_scale(&mut self, scale: f32) {
        self.scale = f32::clamp(scale, SCALE_MIN, SCALE_MAX);
    }
}

fn apply(glyph: &mut Glyph, delta: &Delta) {
    delta.add.iter().for_each(|pixel| {
        glyph.pixels.set(*pixel, true);
    });
    delta.remove.iter().for_each(|pixel| {
        glyph.pixels.set(*pixel, false);
    });
}

fn no_glyph<'a, M>() -> Container<'a, M> {
    container("no glyph")
        .center(Length::Fill)
        .style(container::bordered_box)
}
