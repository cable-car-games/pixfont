// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use iced::{Element, Task, widget::Column};

use crate::ui::widgets;

pub struct State {}

#[derive(Debug, Clone)]
pub enum Message {
    Internal(InternalMessage),
}

#[derive(Debug, Clone)]
pub enum InternalMessage {}

pub fn view(_state: &State) -> Element<'_, Message> {
    widgets::settings::wrapper(Column::new().push(widgets::settings::inset(
        Column::new().push(widgets::settings::title("Apperarance")),
    )))
    .into()
}

pub fn update(_state: &mut State, message: InternalMessage) -> Task<Message> {
    match message {}
}
