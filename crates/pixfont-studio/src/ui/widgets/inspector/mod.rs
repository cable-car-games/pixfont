// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use iced::{
    Element, Font, alignment,
    widget::{Column, Container, Row, Text, text::IntoFragment},
};

pub const LABEL_WIDTH: u32 = 120;

pub fn section<'state, M: 'state>(title: impl IntoFragment<'state>) -> Column<'state, M> {
    Column::new().spacing(2).push(header(title))
}

pub fn header<'state, M: 'state>(title: impl IntoFragment<'state>) -> Element<'state, M> {
    Container::new(Text::new(title).font(Font {
        family: iced::font::Family::Name("Razza Sans"),
        weight: iced::font::Weight::Bold,
        ..Default::default()
    }))
    .into()
}

pub fn property<'state, M: 'state>(
    label: impl IntoFragment<'state>,
    field: impl Into<Element<'state, M>>,
) -> Element<'state, M> {
    Row::new()
        .push(Text::new(label).width(LABEL_WIDTH))
        .push(field)
        .align_y(alignment::Vertical::Center)
        .spacing(2)
        .into()
}
