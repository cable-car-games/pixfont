// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use iced::{
    Element, Length, Task, padding,
    widget::{Column, Row, button, container, row, tooltip},
};
use pixicons::icon::icon;

use crate::ui::widgets::tooltip::toolbar_tooltip;

mod directory;
mod font;
mod glyph;

pub struct State {
    collapsed: bool,

    tab: Tab,
    font: font::State,
    directory: directory::State,
    glyph: glyph::State,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Font,
    Directory,
    Glyph,
}

#[derive(Debug, Clone)]
pub enum Message {
    SetCollapsed(bool),
    SetTab(Tab),

    Directory(directory::Message),
}

impl From<Message> for crate::Message {
    fn from(message: Message) -> Self {
        Self::Editor(super::Message::Inspector(message))
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            collapsed: false,
            tab: Tab::Font,
            directory: Default::default(),
            font: Default::default(),
            glyph: Default::default(),
        }
    }
}

impl State {
    pub fn view<'state>(
        &'state self,
        font: &'state pixfont::Font,
        selected_glyph: &Option<String>,
    ) -> Element<'state, crate::Message> {
        let tabs = [
            (Tab::Font, icon!(font), "Font"),
            (Tab::Directory, icon!(glyphset), "Glyphs"),
            (Tab::Glyph, icon!(glyph), "Glyph"),
        ];

        if self.collapsed {
            return container(
                Column::new()
                    .push(tooltip(
                        button(icon!(drawer.left.open))
                            .padding(padding::horizontal(6).vertical(5))
                            .style(button::subtle)
                            .on_press(Message::SetCollapsed(false).into()),
                        container("Expand sidebar")
                            .padding(padding::vertical(4).horizontal(8))
                            .style(container::bordered_box),
                        tooltip::Position::Right,
                    ))
                    .extend(tabs.map(|(tab, icon, name)| {
                        tooltip(
                            button(icon)
                                .style(button::subtle)
                                .padding(padding::horizontal(6).vertical(5))
                                .on_press(Message::SetTab(tab).into()),
                            container(name)
                                .padding(padding::vertical(4).horizontal(8))
                                .style(container::bordered_box),
                            tooltip::Position::Right,
                        )
                        .into()
                    }))
                    .spacing(4)
                    .height(Length::Fill),
            )
            .padding(4)
            .style(container::bordered_box)
            .into();
        }

        Column::new()
            .push(
                container(
                    Row::new()
                        .push(toolbar_tooltip(
                            button(icon!(drawer.left.close))
                                .style(button::subtle)
                                .on_press(Message::SetCollapsed(true).into())
                                .padding(padding::horizontal(6).vertical(5)),
                            "Collapse sidebar",
                        ))
                        .extend(tabs.map(|(tab, icon, name)| {
                            button(container(row![icon, name].spacing(2)).center_x(Length::Fill))
                                .style(if !self.collapsed && self.tab == tab {
                                    button::primary
                                } else {
                                    button::subtle
                                })
                                .width(Length::Fill)
                                .on_press(Message::SetTab(tab).into())
                                .into()
                        }))
                        .spacing(4),
                )
                .padding(4)
                .style(container::bordered_box),
            )
            .push(match self.tab {
                Tab::Font => self.font.view(font),
                Tab::Directory => self.directory.view(font, selected_glyph),
                Tab::Glyph => self.glyph.view(font, selected_glyph),
            })
            .spacing(8)
            .width(320)
            .into()
    }

    pub fn update(&mut self, font: &mut pixfont::Font, message: Message) -> Task<crate::Message> {
        match message {
            Message::SetTab(tab) => {
                self.tab = tab;
                self.collapsed = false;
                Task::none()
            }
            Message::SetCollapsed(collapsed) => {
                self.collapsed = collapsed;
                Task::none()
            }
            Message::Directory(message) => self.directory.update(font, message),
        }
    }
}
