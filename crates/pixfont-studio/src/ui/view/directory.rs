// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use std::fmt::Display;

use iced::{
    Element, Length, Task,
    widget::{
        Button, Column, Container, PickList, Row, Scrollable, TextInput, pane_grid::Axis::Vertical,
    },
};

use crate::ui::widgets::icon::Icon;

pub struct Directory {
    filter: Option<String>,
    order: DirectoryOrder,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DirectoryOrder {
    /// Don't order
    #[default]
    None,

    /// Order by glyph name
    Name,

    /// Order by unicode mapping
    Unicode,
}

impl Display for DirectoryOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            DirectoryOrder::None => "None",
            DirectoryOrder::Name => "Name",
            DirectoryOrder::Unicode => "Unicode",
        })
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectGlyph(String),

    SetFilter(String),
    SetOrder(DirectoryOrder),

    Noop,
}

impl Directory {
    pub fn new() -> Self {
        Self {
            filter: None,
            order: DirectoryOrder::None,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let orders = [
            DirectoryOrder::None,
            DirectoryOrder::Name,
            DirectoryOrder::Unicode,
        ];

        let inspector = Column::new().push("Sidebar").width(320);

        let toolbar = Row::new()
            .push(
                Row::new()
                    .push(
                        Button::new(
                            Row::new()
                                .push(Icon::BiPlusLg.as_svg())
                                .push("New glyph")
                                .spacing(4),
                        )
                        .style(iced::widget::button::subtle)
                        .on_press(Message::Noop),
                    )
                    .push(
                        Button::new(
                            Row::new()
                                .push(Icon::BiPlusLg.as_svg())
                                .push("New from set")
                                .spacing(4),
                        )
                        .style(iced::widget::button::subtle)
                        .on_press(Message::Noop),
                    )
                    .spacing(2),
            )
            .push(
                Container::new(
                    Row::new()
                        .push(
                            TextInput::new(
                                "Search",
                                self.filter.clone().unwrap_or(String::new()).as_str(),
                            )
                            .on_input(|s| Message::SetFilter(s))
                            .width(140),
                        )
                        .push(PickList::new(orders, Some(self.order), |order| {
                            Message::SetOrder(order)
                        }))
                        .spacing(4),
                )
                .align_right(Length::Fill),
            )
            .padding(4);

        let directory = Column::new();

        Row::new()
            .push(Scrollable::new(inspector))
            .push(
                Container::new(
                    Column::new().push(toolbar).push(
                        Scrollable::new(directory)
                            .width(Length::Fill)
                            .height(Length::Fill),
                    ),
                )
                .style(iced::widget::container::bordered_box),
            )
            .into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Noop => Task::none(),
            Message::SetFilter(str) => {
                self.filter = if str.is_empty() {
                    None
                } else {
                    Some(str.clone())
                };
                Task::none()
            }
            Message::SetOrder(order) => {
                self.order = order;
                Task::none()
            }
            _ => todo!(),
        }
    }
}
