// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use std::{collections::LinkedList, ops::Not};

use iced::{
    Element, Length, Task, Vector,
    alignment::Vertical,
    widget::{Button, Column, Container, Row, Scrollable, Space, Text, button, text, text_input},
};
use iced_aw::number_input;
use pixfont::{Glyph, Guideline, Guidelines};

use crate::{
    settings::Settings,
    ui::widgets::{
        glyph_editor::{Delta, GlyphEditor, Tool},
        icon::Icon,
        inspector,
    },
};

pub struct Editor {
    scale: f32,
    offset: Vector<f32>,
    tool: Tool,

    undo_stack: LinkedList<Delta>,
    redo_stack: LinkedList<Delta>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Message {
    Apply(Delta),
    SetScale(f32),
    SetOffset(Vector<f32>),
    SetTool(Tool),
    SetGlyphProp(GlyphProp),
    SetGuideline(GuidelineAction),
    Undo,
    Redo,
    Copy,
    Paste,
    ResetViewport,
    ZoomIn,
    ZoomOut,
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

const SCALE_MIN: f32 = 2.0;
const SCALE_MAX: f32 = 128.0;
const DEFAULT_SCALE: f32 = 16.0;

impl Default for Editor {
    fn default() -> Self {
        Self {
            scale: DEFAULT_SCALE,
            offset: Default::default(),
            tool: Tool::Pen,
            undo_stack: LinkedList::new(),
            redo_stack: LinkedList::new(),
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

        let codepoint = font.get_glyph_codepoint(selected_glyph_name);

        let inspector = Column::new()
            .width(320)
            .spacing(8)
            .push(
                inspector::section("Glyph")
                    .push(inspector::property("Glyph name", text(selected_glyph_name)))
                    .push(inspector::property(
                        "Codepoint",
                        text(codepoint.map_or("???".to_string(), |(codepoint, _)| {
                            format!("U+{:04X}", codepoint)
                        })),
                    ))
                    .push(inspector::property(
                        "Alternate",
                        text(codepoint.map_or("???".to_string(), |(_, alternate)| {
                            alternate.unwrap_or(&"(primary)".to_string()).clone()
                        })),
                    ))
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
                        number_input(&font.metrics.ascender, 1..i32::MAX, |value| {
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
                            .style(iced::widget::button::subtle)
                            .on_press_maybe(
                                self.undo_stack.is_empty().not().then_some(Message::Undo),
                            ),
                    )
                    .push(
                        Button::new(Icon::BiArrowClockwise.as_svg())
                            .style(iced::widget::button::subtle)
                            .on_press_maybe(
                                self.redo_stack.is_empty().not().then_some(Message::Redo),
                            ),
                    )
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
                            (Tool::Pen, Icon::BiPen, "Pen"),
                            (Tool::Line, Icon::BiSlashLg, "Line"),
                            (Tool::Rectangle, Icon::BiSquare, "Rectangle"),
                            //(Tool::Fill, Icon::BiPaintBucket, "Fill"),
                            (Tool::Eraser, Icon::BiEraser, "Eraser"),
                            (Tool::Pan, Icon::BiArrowsMove, "Pan"),
                        ]
                        .map(|(tool, icon, _name)| {
                            Button::new(icon.as_svg())
                                .on_press(Message::SetTool(tool))
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
                        .align_y(Vertical::Center)
                        .push(
                            Button::new(Icon::BiBorderInner.as_svg())
                                .style(iced::widget::button::subtle)
                                .on_press(Message::ResetViewport),
                        )
                        .push(
                            Button::new(Icon::BiZoomIn.as_svg())
                                .style(iced::widget::button::subtle)
                                .on_press(Message::ZoomIn),
                        )
                        .push(text(format!("{:.0}%", self.scale * 100.0)))
                        .push(
                            Button::new(Icon::BiZoomOut.as_svg())
                                .style(iced::widget::button::subtle)
                                .on_press(Message::ZoomOut),
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
                                .on_scale(Message::SetScale)
                                .on_pan(Message::SetOffset)
                                .on_tool(Message::SetTool)
                                .on_apply(Message::Apply),
                        )
                        .spacing(4),
                )
                .style(iced::widget::container::bordered_box)
                .padding(4),
            )
            .into()
    }

    pub fn update(
        &mut self,
        message: Message,
        font: &mut pixfont::Font,
        glyph: Option<&String>,
    ) -> Task<Message> {
        let glyph = glyph.expect("bug: no glyph selected");
        let glyph = font
            .glyphs
            .get_mut(glyph)
            .expect("bug: glyph does not exist");

        match message {
            Message::Apply(delta) => {
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

            Message::SetGlyphProp(glyph_prop) => {
                match glyph_prop {
                    GlyphProp::Advance(value) => glyph.advance = value,

                    // TODO: the ones below should probably be moved elsewhere
                    GlyphProp::Ascender(value) => font.metrics.ascender = value,
                    GlyphProp::Descender(value) => font.metrics.descender = value,
                    GlyphProp::CapHeight(value) => font.metrics.cap_height = value,
                    GlyphProp::XHeight(value) => font.metrics.x_height = value,
                };
                Task::none()
            }

            Message::SetGuideline(guideline_action) => match guideline_action {
                GuidelineAction::Create => {
                    glyph.guidelines.x.push(pixfont::Guideline {
                        name: "".to_string(),
                        position: 16,
                    });

                    Task::none()
                }

                GuidelineAction::Rename {
                    scope,
                    direction,
                    index,
                    name,
                } => {
                    let scope = match scope {
                        GuidelineScope::Global => &mut font.metrics.guidelines,
                        GuidelineScope::Local => &mut glyph.guidelines,
                    };

                    let guideline = direction_of(scope, direction)
                        .get_mut(index)
                        .expect("bug: index does not exist");

                    guideline.name = name;
                    Task::none()
                }
                GuidelineAction::Set {
                    scope,
                    direction,
                    index,
                    position,
                } => {
                    let scope = match scope {
                        GuidelineScope::Global => &mut font.metrics.guidelines,
                        GuidelineScope::Local => &mut glyph.guidelines,
                    };

                    let guideline = direction_of(scope, direction)
                        .get_mut(index)
                        .expect("bug: index does not exist");

                    guideline.position = position;
                    Task::none()
                }

                GuidelineAction::Remove {
                    scope,
                    direction,
                    index,
                } => {
                    let scope = match scope {
                        GuidelineScope::Global => &mut font.metrics.guidelines,
                        GuidelineScope::Local => &mut glyph.guidelines,
                    };

                    direction_of(scope, direction).remove(index);
                    Task::none()
                }

                GuidelineAction::MakeGlobal { direction, index } => {
                    direction_of(&mut font.metrics.guidelines, direction)
                        .push(direction_of(&mut glyph.guidelines, direction).remove(index));
                    Task::none()
                }

                GuidelineAction::MakeLocal { direction, index } => {
                    direction_of(&mut glyph.guidelines, direction)
                        .push(direction_of(&mut font.metrics.guidelines, direction).remove(index));
                    Task::none()
                }

                GuidelineAction::MakeX { scope, index } => {
                    let scope = match scope {
                        GuidelineScope::Global => &mut font.metrics.guidelines,
                        GuidelineScope::Local => &mut glyph.guidelines,
                    };

                    scope.x.push(scope.y.remove(index));
                    Task::none()
                }
                GuidelineAction::MakeY { scope, index } => {
                    let scope = match scope {
                        GuidelineScope::Global => &mut font.metrics.guidelines,
                        GuidelineScope::Local => &mut glyph.guidelines,
                    };

                    scope.y.push(scope.x.remove(index));
                    Task::none()
                }
            },

            Message::Undo => {
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
        }
    }

    fn set_scale(&mut self, scale: f32) {
        self.scale = f32::clamp(scale, SCALE_MIN, SCALE_MAX);
    }
}

fn direction_of(guidelines: &mut Guidelines, direction: GuidelineDirection) -> &mut Vec<Guideline> {
    match direction {
        GuidelineDirection::X => &mut guidelines.x,
        GuidelineDirection::Y => &mut guidelines.y,
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
