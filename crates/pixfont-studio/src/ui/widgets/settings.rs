// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use iced::{
    Element, Length,
    alignment::Vertical,
    padding,
    widget::{Button, Column, Row, Text, container, space, text::IntoFragment},
};
use pixicons::icon::Icon;

use crate::ui::font::razza;

pub fn wrapper<'state, Message: 'state>(
    content: impl Into<Element<'state, Message>>,
) -> Element<'state, Message> {
    container(content)
        .align_left(Length::Fill)
        .align_top(Length::Fill)
        .padding(padding::vertical(16).horizontal(8))
        .max_width(640)
        .into()
}

pub fn inset<'state, Message: 'state>(
    content: impl Into<Element<'state, Message>>,
) -> Element<'state, Message> {
    container(content).padding(padding::horizontal(12)).into()
}

pub fn title<'a>(label: &'a str) -> Text<'a> {
    Text::new(label).size(24).font(razza::BOLD)
}

pub fn section<'a, M: 'a>(title: impl IntoFragment<'a>) -> Column<'a, M> {
    Column::with_capacity(5)
        .spacing(4)
        .push(section_title(title))
}

pub fn section_title<'a>(label: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(label).size(16).font(razza::BOLD)
}

pub fn button<'state, Message: 'state>(
    title: impl IntoFragment<'state>,
    subtitle: Option<impl IntoFragment<'state>>,
    icon: Option<Icon<'state>>,
) -> Button<'state, Message> {
    let mut label = Column::new().push(Text::new(title));
    if let Some(subtitle) = subtitle {
        label = label.push(Text::new(subtitle).size(12));
    }

    let mut row = Row::new();
    if let Some(icon) = icon {
        row = row.push(icon);
    }

    let row = row
        .push(label)
        .push(space::horizontal())
        .align_y(Vertical::Center);

    Button::new(row)
        .style(iced::widget::button::background)
        .padding(padding::all(12).vertical(8))
}
