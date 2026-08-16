// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use iced::{
    Element, Length,
    widget::{Column, Row, button, container, row, scrollable, text, text_input},
};
use iced_aw::number_input;
use pixfont::{Guideline, Guidelines};
use ucd::Codepoint;

use crate::{
    project::{self, Direction, GlyphAction, GuidelineAction, SetMetrics},
    ui::widgets::inspector,
};

#[derive(Debug, Default)]
pub struct State {}

impl State {
    pub fn view<'a>(
        &'a self,
        font: &'a pixfont::Font,
        selected_glyph: &Option<String>,
    ) -> Element<'a, crate::Message> {
        let glyph = selected_glyph
            .as_ref()
            .map(|name| font.glyphs.get(name).expect("bug: glyph does not exist"));

        if let Some(glyph) = glyph {
            let codepoint = font
                .get_glyph_codepoint(&glyph.name)
                .map(|(codepoint, _)| codepoint)
                .expect("bug: glyph has no codepoint");

            container(
                scrollable(
                    Column::new()
                        .push(
                            inspector::section("Glyph")
                                .push(inspector::property("Glyph name", text(glyph.name.clone())))
                                .push(inspector::property(
                                    "Codepoint",
                                    text(format!("U+{codepoint:04X}")),
                                ))
                                .push(inspector::property(
                                    "Codepoint name",
                                    Codepoint::of(codepoint).map_or("unknown", |cp| cp.name),
                                )),
                        )
                        .push(
                            inspector::section("Metrics")
                                .push(inspector::property(
                                    "Advance",
                                    number_input(&glyph.advance, 0..(i32::MAX as u32), |value| {
                                        project::Action::Glyph {
                                            name: glyph.name.clone(),
                                            action: GlyphAction::SetAdvance(value),
                                        }
                                        .into()
                                    })
                                    .width(Length::Fill),
                                ))
                                .push(inspector::property(
                                    "Ascender",
                                    number_input(&font.metrics.ascender, 1..=i32::MAX, |value| {
                                        project::Action::Metrics(project::SetMetrics::SetAscender(
                                            value,
                                        ))
                                        .into()
                                    })
                                    .width(Length::Fill),
                                ))
                                .push(inspector::property(
                                    "Descender",
                                    number_input(&font.metrics.descender, i32::MIN..=0, |value| {
                                        project::Action::Metrics(project::SetMetrics::SetDescender(
                                            value,
                                        ))
                                        .into()
                                    })
                                    .width(Length::Fill),
                                ))
                                .push(inspector::property(
                                    "Cap height",
                                    number_input(&font.metrics.cap_height, 1..i32::MAX, |value| {
                                        project::Action::Metrics(project::SetMetrics::SetCapHeight(
                                            value,
                                        ))
                                        .into()
                                    })
                                    .width(Length::Fill),
                                ))
                                .push(inspector::property(
                                    "x height",
                                    number_input(&font.metrics.x_height, 1..i32::MAX, |value| {
                                        project::Action::Metrics(project::SetMetrics::SetXHeight(
                                            value,
                                        ))
                                        .into()
                                    })
                                    .width(Length::Fill),
                                )),
                        )
                        .push(
                            inspector::section("Guidelines")
                                .extend(guidelines(
                                    &font.metrics.guidelines,
                                    Direction::X,
                                    |action| {
                                        project::Action::Metrics(SetMetrics::Guideline(
                                            Direction::X,
                                            action,
                                        ))
                                        .into()
                                    },
                                ))
                                .extend(guidelines(
                                    &font.metrics.guidelines,
                                    Direction::Y,
                                    |action| {
                                        project::Action::Metrics(SetMetrics::Guideline(
                                            Direction::Y,
                                            action,
                                        ))
                                        .into()
                                    },
                                ))
                                .extend(guidelines(&glyph.guidelines, Direction::X, |action| {
                                    project::Action::Glyph {
                                        name: glyph.name.to_string(),
                                        action: GlyphAction::Guideline(Direction::X, action),
                                    }
                                    .into()
                                }))
                                .extend(guidelines(&glyph.guidelines, Direction::Y, |action| {
                                    project::Action::Glyph {
                                        name: glyph.name.to_string(),
                                        action: GlyphAction::Guideline(Direction::Y, action),
                                    }
                                    .into()
                                }))
                                .push(
                                    row([
                                        button(container("New global").center_x(Length::Fill))
                                            .style(button::background)
                                            .on_press(
                                                project::Action::Metrics(SetMetrics::Guideline(
                                                    project::Direction::X,
                                                    project::GuidelineAction::Create {
                                                        name: "".to_string(),
                                                        position: 16,
                                                    },
                                                ))
                                                .into(),
                                            )
                                            .into(),
                                        button(container("New local").center_x(Length::Fill))
                                            .style(button::background)
                                            .on_press(
                                                project::Action::Glyph {
                                                    name: glyph.name.to_string(),
                                                    action: GlyphAction::Guideline(
                                                        project::Direction::X,
                                                        project::GuidelineAction::Create {
                                                            name: "".to_string(),
                                                            position: 16,
                                                        },
                                                    ),
                                                }
                                                .into(),
                                            )
                                            .into(),
                                    ])
                                    .spacing(2),
                                ), //.extend([].into_iter().flat_map(|(vec)| {})),
                        )
                        .push(inspector::extra_section(&glyph.extra, |extra| {
                            project::Action::Glyph {
                                name: glyph.name.to_string(),
                                action: GlyphAction::Extra(extra),
                            }
                            .into()
                        }))
                        .spacing(12)
                        .width(Length::Fill)
                        .height(Length::Fill),
                )
                .anchor_top()
                .height(Length::Fill),
            )
        } else {
            container(
                Column::new()
                    .push("You haven't selected a glyph.")
                    .push(button("Glyphs directory").on_press(crate::Message::Editor(
                        crate::ui::pages::editor::Message::Inspector(super::Message::SetTab(
                            super::Tab::Directory,
                        )),
                    )))
                    .spacing(8),
            )
            .center(Length::Fill)
        }
        .padding(8)
        .style(container::bordered_box)
        .into()
    }
}

fn guidelines<'a>(
    guidelines: &'a Guidelines,
    direction: Direction,
    message: impl Fn(GuidelineAction) -> crate::Message + Copy + 'a,
) -> impl Iterator<Item = Element<'a, crate::Message>> {
    let guidelines = match direction {
        Direction::X => &guidelines.x,
        Direction::Y => &guidelines.y,
    };

    let row = move |index, guideline: &'a Guideline| {
        Row::<'a, crate::Message>::with_capacity(2)
            .push(
                text_input("(name)", &guideline.name)
                    .width(inspector::LABEL_WIDTH)
                    .on_input(move |name| message(GuidelineAction::SetName { index, name })),
            )
            .push(
                Row::new()
                    .push(
                        number_input(&guideline.position, i32::MIN..=i32::MAX, move |position| {
                            message(GuidelineAction::SetPosition { index, position })
                        })
                        .width(Length::Fill),
                    )
                    .push(
                        button(match direction {
                            Direction::X => "X",
                            Direction::Y => "Y",
                        })
                        .on_press(message(GuidelineAction::SetDirection {
                            index,
                            direction: match direction {
                                Direction::X => Direction::Y,
                                Direction::Y => Direction::X,
                            },
                        }))
                        .style(button::background),
                    )
                    .push(
                        button("\u{00D7}")
                            .style(button::danger)
                            .on_press(message(GuidelineAction::Remove { index })),
                    )
                    .spacing(2),
            )
            .spacing(2)
            .into()
    };

    guidelines
        .iter()
        .enumerate()
        .map(move |(index, guideline)| row(index, guideline))
}
