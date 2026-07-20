// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use iced::{
    Element, Font, Length, alignment,
    widget::{Column, Container, Row, Text, button, text, text_input},
};

use crate::project::Extra;

pub const LABEL_WIDTH: u32 = 120;

pub fn section<'state, M: 'state>(title: impl text::IntoFragment<'state>) -> Column<'state, M> {
    Column::new().spacing(2).push(header(title))
}

pub fn header<'state, M: 'state>(title: impl text::IntoFragment<'state>) -> Element<'state, M> {
    Container::new(Text::new(title).font(Font {
        weight: iced::font::Weight::Bold,
        ..Default::default()
    }))
    .into()
}

pub fn property<'state, M: 'state>(
    label: impl text::IntoFragment<'state>,
    field: impl Into<Element<'state, M>>,
) -> Element<'state, M> {
    Row::with_capacity(2)
        .push(Text::new(label).width(LABEL_WIDTH))
        .push(field)
        .align_y(alignment::Vertical::Center)
        .spacing(2)
        .into()
}

pub fn extra_section<'state>(
    font: &'state pixfont::Font,
    map_extra: impl Fn(Extra) -> crate::Message + Clone + 'state,
) -> Element<'state, crate::Message> {
    let mut section = section("Extra");

    for (key, value) in &font.metadata.extra {
        let set_key = map_extra.clone();
        let set_key = move |s| {
            set_key(Extra::RenameKey {
                old: key.clone(),
                new: s,
            })
        };

        let set_value = map_extra.clone();
        let set_value = move |value| {
            set_value(Extra::SetValue {
                key: key.clone(),
                value,
            })
        };

        section = section.push(
            Row::new()
                .push(
                    text_input("(key)", key)
                        .width(LABEL_WIDTH)
                        .on_input(set_key),
                )
                .push(text_input("(value)", value).on_input(set_value))
                .push(
                    button("\u{00D7}")
                        .style(button::danger)
                        .on_press(map_extra(Extra::Remove { key: key.clone() })),
                )
                .spacing(2),
        );
    }

    section
        .push(
            button(
                text("Add extra data")
                    .align_x(alignment::Horizontal::Center)
                    .width(Length::Fill),
            )
            .style(button::background)
            .on_press(map_extra(Extra::Add {
                key: "".to_string(),
                value: "".to_string(),
            })),
        )
        .spacing(2)
        .into()
}
