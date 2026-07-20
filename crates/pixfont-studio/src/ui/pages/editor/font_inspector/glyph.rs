// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use iced::{
    Element, Length,
    widget::{Column, container, scrollable, text},
};
use iced_aw::number_input;
use ucd::Codepoint;

use crate::{
    project::{self, GlyphAction},
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

        //let Some(selected_glyph_name) = selected_glyph_name else {
        //    // TODO: disable the tab while a glyph is not selected, or select a default gltph
        //    return Text::new("(no glyph selected)").into();
        //};

        //let Some(glyph) = font.glyphs.get(selected_glyph_name) else {
        //    // this should ideally never happen, but we check for it anyway
        //    return Text::new("(glyph does not exist)").into();
        //};

        //let inspector = Column::new()
        //    .width(320)
        //    .spacing(8)
        //    .push(
        //        inspector::section("Glyph")
        //            .push(inspector::property("Glyph name", text(selected_glyph_name)))
        //            .push(inspector::property(
        //                "Codepoint",
        //                text(codepoint.map_or("???".to_string(), |(codepoint, _)| {
        //                    format!("U+{:04X}", codepoint)
        //                })),
        //            ))
        //            .push(inspector::property(
        //                "Codepoint name",
        //                text(codepoint.map_or("???", |(codepoint, _)| {
        //                    Codepoint::of(codepoint).map_or("???", |codepoint| codepoint.name)
        //                })),
        //            ))
        //            .push(inspector::property(
        //                "Alternate",
        //                text(codepoint.map_or("???".to_string(), |(_, alternate)| {
        //                    alternate.unwrap_or(&"(primary)".to_string()).clone()
        //                })),
        //            ))
        //            .push(inspector::property(
        //                "Advance",
        //                number_input(&glyph.advance, 0..(i32::MAX as u32), |value| {
        //                    Message::SetGlyphProp(GlyphProp::Advance(value))
        //                }),
        //            )),
        //    )
        //    .push(
        //        inspector::section("Metrics")
        //            .push(inspector::property(
        //                "Em height",
        //                text((font.metrics.ascender - font.metrics.descender).to_string()),
        //            ))
        //            .push(inspector::property(
        //                "Ascender",
        //                number_input(&font.metrics.ascender, 1..i32::MAX, |value| {
        //                    Message::SetGlyphProp(GlyphProp::Ascender(value))
        //                })
        //                .width(Length::Fill),
        //            ))
        //            .push(inspector::property(
        //                "Descender",
        //                number_input(&font.metrics.descender, i32::MIN..0, |value| {
        //                    Message::SetGlyphProp(GlyphProp::Descender(value))
        //                })
        //                .width(Length::Fill),
        //            ))
        //            .push(inspector::property(
        //                "Cap height",
        //                number_input(&font.metrics.cap_height, 0..i32::MAX, |value| {
        //                    Message::SetGlyphProp(GlyphProp::CapHeight(value))
        //                })
        //                .width(Length::Fill),
        //            ))
        //            .push(inspector::property(
        //                "x height",
        //                number_input(&font.metrics.x_height, 0..i32::MAX, |value| {
        //                    Message::SetGlyphProp(GlyphProp::XHeight(value))
        //                })
        //                .width(Length::Fill),
        //            )),
        //    )
        //    .push(
        //        inspector::section("Guidelines")
        //            .extend(
        //                [
        //                    (
        //                        GuidelineScope::Global,
        //                        GuidelineDirection::X,
        //                        &font.metrics.guidelines.x,
        //                    ),
        //                    (
        //                        GuidelineScope::Global,
        //                        GuidelineDirection::Y,
        //                        &font.metrics.guidelines.y,
        //                    ),
        //                    (
        //                        GuidelineScope::Local,
        //                        GuidelineDirection::X,
        //                        &glyph.guidelines.x,
        //                    ),
        //                    (
        //                        GuidelineScope::Local,
        //                        GuidelineDirection::Y,
        //                        &glyph.guidelines.y,
        //                    ),
        //                ]
        //                .into_iter()
        //                .flat_map(|(scope, direction, vec)| {
        //                    vec.iter().enumerate().map(move |(index, guideline)| {
        //                        Row::new()
        //                            .spacing(2)
        //                            .push(text_input("(key)", &guideline.name).on_input(
        //                                move |name| {
        //                                    Message::SetGuideline(GuidelineAction::Rename {
        //                                        scope,
        //                                        direction,
        //                                        index,
        //                                        name,
        //                                    })
        //                                },
        //                            ))
        //                            .push(number_input(
        //                                &guideline.position,
        //                                i32::MIN..i32::MAX,
        //                                move |value| {
        //                                    Message::SetGuideline(GuidelineAction::Set {
        //                                        scope,
        //                                        direction,
        //                                        index,
        //                                        position: value,
        //                                    })
        //                                },
        //                            ))
        //                            .push(
        //                                button(match direction {
        //                                    GuidelineDirection::X => "X",
        //                                    GuidelineDirection::Y => "Y",
        //                                })
        //                                .style(iced::widget::button::subtle)
        //                                .on_press(
        //                                    Message::SetGuideline(match direction {
        //                                        GuidelineDirection::X => {
        //                                            GuidelineAction::MakeY { scope, index }
        //                                        }
        //                                        GuidelineDirection::Y => {
        //                                            GuidelineAction::MakeX { scope, index }
        //                                        }
        //                                    }),
        //                                ),
        //                            )
        //                            .push(
        //                                button(match scope {
        //                                    GuidelineScope::Global => "G",
        //                                    GuidelineScope::Local => "g",
        //                                })
        //                                .on_press(
        //                                    Message::SetGuideline(match scope {
        //                                        GuidelineScope::Global => {
        //                                            GuidelineAction::MakeLocal { direction, index }
        //                                        }
        //                                        GuidelineScope::Local => {
        //                                            GuidelineAction::MakeGlobal { direction, index }
        //                                        }
        //                                    }),
        //                                ),
        //                            )
        //                            .push(button("×").style(iced::widget::button::danger).on_press(
        //                                Message::SetGuideline(GuidelineAction::Remove {
        //                                    scope,
        //                                    direction,
        //                                    index,
        //                                }),
        //                            ))
        //                            .into()
        //                    })
        //                }),
        //            )
        //            .push(
        //                button("Add guideline")
        //                    .style(iced::widget::button::subtle)
        //                    .on_press(Message::SetGuideline(GuidelineAction::Create)),
        //            ),
        //    );

        container(
            scrollable(if let Some(glyph) = glyph {
                let codepoint = font
                    .get_glyph_codepoint(&glyph.name)
                    .map(|(codepoint, _)| codepoint)
                    .expect("bug: glyph has no codepoint");

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
                                number_input(&font.metrics.ascender, 1..i32::MAX, |value| {
                                    project::Action::Metrics(project::SetMetrics::SetAscender(
                                        value,
                                    ))
                                    .into()
                                })
                                .width(Length::Fill),
                            ))
                            .push(inspector::property(
                                "Descender",
                                number_input(&font.metrics.descender, i32::MIN..0, |value| {
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
                                    project::Action::Metrics(project::SetMetrics::SetXHeight(value))
                                        .into()
                                })
                                .width(Length::Fill),
                            )),
                    )
                    .push(
                        inspector::section("Guidelines"), //.extend([].into_iter().flat_map(|(vec)| {})),
                    )
                    .padding(8)
                    .spacing(12)
                    .into()
            } else {
                Into::<Element<'a, crate::Message>>::into(
                    text("no glyph").width(Length::Fill).height(Length::Fill),
                )
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .anchor_top(),
        )
        .style(container::bordered_box)
        .into()
    }
}
