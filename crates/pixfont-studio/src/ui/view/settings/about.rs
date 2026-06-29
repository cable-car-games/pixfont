// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use iced::{Element, Task, widget::column};

use crate::ui::widgets::{self};

pub struct State {}

#[derive(Debug, Clone)]
pub enum Message {
    ShowLicense,
    Internal(InternalMessage),
}

#[derive(Debug, Clone)]
pub enum InternalMessage {
    OpenWebsite,
    OpenSource,
}

pub fn view(_: &State) -> Element<'_, Message> {
    widgets::settings::wrapper(
        column!(
            widgets::settings::inset(
                column!(
                    widgets::settings::title("PixFont Studio"),
                    env!("CARGO_PKG_VERSION"),
                    "Cable Car Games",
                )
                .spacing(2)
            ),
            column![
                widgets::settings::button(
                    "itch.io",
                    Some("GNU Affero General Public License 3.0 or later"),
                    None
                )
                .on_press(Message::Internal(InternalMessage::OpenWebsite)),
                widgets::settings::button("GitHub", Some("cable-car-games/pixfont"), None)
                    .on_press(Message::Internal(InternalMessage::OpenSource)),
                widgets::settings::button(
                    "Software License",
                    Some("GNU Affero General Public License 3.0 or later"),
                    None
                )
                .on_press(Message::ShowLicense)
            ]
            .spacing(2),
        )
        .spacing(8),
    )
}

pub fn update(_: &mut State, message: InternalMessage) -> Task<Message> {
    match message {
        InternalMessage::OpenWebsite => {
            let _ = open::that("https://cable-car.itch.io/pixfont");
            Task::none()
        }
        InternalMessage::OpenSource => {
            let _ = open::that("https://github.com/cable-car-games/pixfont");
            Task::none()
        }
    }
}
