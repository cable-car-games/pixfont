// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use iced::{
    Element, Length, Task,
    widget::{
        Button, Column, Container, Row, Scrollable,
        scrollable::{Direction, Scrollbar},
    },
};

use crate::ui::widgets::icon::Icon;

mod about;
mod appearance;

pub struct Settings {
    page: Page,
    appearance: appearance::State,
    about: about::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    Private(PrivateMessage),
}

#[derive(Debug, Clone)]
pub enum PrivateMessage {
    SetPage(Page),

    // pages
    Appearance(appearance::Message),
    About(about::Message),
}

impl From<PrivateMessage> for Message {
    fn from(value: PrivateMessage) -> Self {
        Self::Private(value)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            page: Page::Appearance,
            appearance: Default::default(),
            about: about::State {},
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Page {
    Appearance,
    About,
}

impl Settings {
    pub fn view<'state>(
        &'state self,
        settings: &'state crate::settings::Settings,
    ) -> Element<'state, Message> {
        let page_selector = Column::from_iter(
            [
                (Page::Appearance, Icon::BiPalette, "Appearance"),
                (Page::About, Icon::BiInfoCircleFill, "About"),
            ]
            .map(|(page, icon, label)| {
                Button::new(Row::new().push(icon.as_svg()).push(label).spacing(8))
                    .on_press(Message::Private(PrivateMessage::SetPage(page)))
                    .style(if page == self.page {
                        iced::widget::button::primary
                    } else {
                        iced::widget::button::background
                    })
                    .width(Length::Fill)
                    .into()
            }),
        )
        .width(180)
        .spacing(2);

        Row::new()
            .push(Scrollable::new(page_selector))
            .push(
                Container::new(
                    Scrollable::with_direction(
                        Container::new(match self.page {
                            Page::Appearance => {
                                appearance::view(&self.appearance, &settings.appearance)
                                    .map(Self::map_appearance)
                            }
                            Page::About => about::view(&self.about).map(Self::map_about),
                        })
                        .center_x(Length::Fill),
                        Direction::Vertical(Scrollbar::new()),
                    )
                    .anchor_top(),
                )
                .align_top(Length::Fill)
                .style(iced::widget::container::bordered_box),
            )
            .spacing(8)
            .into()
    }

    pub fn update(
        &mut self,
        settings: &mut crate::settings::Settings,
        message: PrivateMessage,
    ) -> Task<Message> {
        match message {
            PrivateMessage::SetPage(page) => {
                self.page = page;
                Task::none()
            }
            PrivateMessage::Appearance(message) => match message {
                appearance::Message::SettingChanged => {
                    settings.save().expect("Failed to save setting");
                    Task::none()
                }
                appearance::Message::Internal(message) => {
                    appearance::update(&mut self.appearance, &mut settings.appearance, message)
                        .map(|message| Message::Private(PrivateMessage::Appearance(message)))
                }
            },
            PrivateMessage::About(message) => match message {
                about::Message::ShowLicense => todo!(),
                about::Message::Internal(message) => about::update(&mut self.about, message)
                    .map(|message| Message::Private(PrivateMessage::About(message))),
            },
        }
    }

    fn map_appearance(m: appearance::Message) -> Message {
        Message::Private(PrivateMessage::Appearance(m))
    }

    fn map_about(m: about::Message) -> Message {
        Message::Private(PrivateMessage::About(m))
    }
}
