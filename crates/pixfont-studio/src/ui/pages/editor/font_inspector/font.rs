// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use iced::{
    Element, Length,
    widget::{Column, container, scrollable, text_input},
};

use crate::{
    project::{self, Extra, SetMetadata},
    ui::widgets::inspector,
};

#[derive(Debug, Default)]
pub struct State {}

impl State {
    pub fn view<'a>(&'a self, font: &'a pixfont::Font) -> Element<'a, crate::Message> {
        container(
            scrollable(
                Column::new()
                    .push(
                        inspector::section("Metadata")
                            .push(inspector::property(
                                "Name",
                                text_input("", &font.metadata.name)
                                    .on_input(set_metadata_fn(SetMetadata::SetName)),
                            ))
                            .push(inspector::property(
                                "Family",
                                text_input("", &font.metadata.family)
                                    .on_input(set_metadata_fn(SetMetadata::SetFamily)),
                            ))
                            .push(inspector::property(
                                "Weight",
                                text_input("", &font.metadata.weight)
                                    .on_input(set_metadata_fn(SetMetadata::SetWeight)),
                            ))
                            .push(inspector::property(
                                "Style",
                                text_input("", &font.metadata.style)
                                    .on_input(set_metadata_fn(SetMetadata::SetStyle)),
                            ))
                            .push(inspector::property(
                                "Author",
                                text_input("", &font.metadata.author)
                                    .on_input(set_metadata_fn(SetMetadata::SetAuthor)),
                            ))
                            .push(inspector::property(
                                "Copyright",
                                text_input("", &font.metadata.copyright)
                                    .on_input(set_metadata_fn(SetMetadata::SetCopyright)),
                            ))
                            .push(inspector::property(
                                "Licence",
                                text_input("", &font.metadata.license)
                                    .on_input(set_metadata_fn(SetMetadata::SetLicense)),
                            )),
                    )
                    .push(inspector::extra_section(&font.metadata.extra, extra_msg))
                    .padding(8)
                    .spacing(12),
            )
            .anchor_top(),
        )
        .style(container::bordered_box)
        .align_top(Length::Fill)
        .into()
    }
}

fn set_metadata_fn(f: impl Fn(String) -> SetMetadata) -> impl Fn(String) -> crate::Message {
    move |value| project::Action::Metadata(f(value)).into()
}

fn extra_msg(action: Extra) -> crate::Message {
    project::Action::Metadata(SetMetadata::Extra(action)).into()
}
