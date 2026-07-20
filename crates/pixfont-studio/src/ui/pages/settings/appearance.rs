// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use iced::{
    Color, Element, Length, Task,
    alignment::Vertical,
    widget::{Column, Row, button, checkbox, container, pick_list},
};
use iced_aw::color_picker;

use crate::{
    settings::{Appearance, Theme},
    ui::widgets::{self, glyph_editor},
};

#[derive(Default)]
pub struct State {
    opened_picker: Option<EditorToken>,
}

#[derive(Debug, Clone)]
pub enum Message {
    SettingChanged,
    Internal(InternalMessage),
}

#[derive(Debug, Clone)]
pub enum InternalMessage {
    SetTheme(Theme),
    SetEditorColor {
        token: EditorToken,
        color: Option<Color>,
    },

    ShowPicker(Option<EditorToken>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorToken {
    Background,
    Gridlines,
    Glyph,
    Origin,
    Metrics,
    Guidelines,
}

pub fn view<'state>(
    state: &'state State,
    appearance: &'state Appearance,
) -> Element<'state, Message> {
    widgets::settings::wrapper(
        Column::new().push(widgets::settings::inset(
            Column::new()
                .spacing(12)
                .push(
                    Column::new()
                        .spacing(4)
                        .push(widgets::settings::title("Appearance"))
                        .push(
                            Row::new()
                                .push(
                                    container("Theme")
                                        .align_left(Length::Fill)
                                        .center_y(Length::Fill),
                                )
                                .push(pick_list(
                                    crate::settings::Theme::ALL,
                                    Some(appearance.theme),
                                    |value| Message::Internal(InternalMessage::SetTheme(value)),
                                )),
                        ),
                )
                .push(
                    widgets::settings::section("Editor colours")
                        .push(color_token(
                            state,
                            "Background",
                            EditorToken::Background,
                            appearance.editor.background.map(Into::into),
                        ))
                        .push(color_token(
                            state,
                            "Gridlines",
                            EditorToken::Gridlines,
                            appearance.editor.gridlines.map(Into::into),
                        ))
                        .push(color_token(
                            state,
                            "Glyph",
                            EditorToken::Glyph,
                            appearance.editor.glyph.map(Into::into),
                        ))
                        .push(color_token(
                            state,
                            "Origin lines",
                            EditorToken::Origin,
                            appearance.editor.origin.map(Into::into),
                        ))
                        .push(color_token(
                            state,
                            "Metric lines",
                            EditorToken::Metrics,
                            appearance.editor.metrics.map(Into::into),
                        ))
                        .push(color_token(
                            state,
                            "Guidelines",
                            EditorToken::Guidelines,
                            appearance.editor.guidelines.map(Into::into),
                        )),
                ),
        )),
    )
}

fn color_token<'state>(
    state: &'state State,
    label: &'state str,
    token: EditorToken,
    value: Option<Color>,
) -> Element<'state, Message> {
    let mut fields = Row::new()
        .spacing(4)
        .align_y(Vertical::Center)
        .push(container(checkbox(value.is_some()).on_toggle(
            move |enable| {
                Message::Internal(InternalMessage::SetEditorColor {
                    token,
                    color: if enable {
                        Some(match token {
                            EditorToken::Background => glyph_editor::BACKGROUND_COLOR,
                            EditorToken::Gridlines => glyph_editor::GRIDLINE_COLOR,
                            EditorToken::Glyph => glyph_editor::GLYPH_COLOR,
                            EditorToken::Origin => glyph_editor::ORIGIN_COLOR,
                            EditorToken::Metrics => glyph_editor::METRICS_COLOR,
                            EditorToken::Guidelines => glyph_editor::GUIDELINE_COLOR,
                        })
                    } else {
                        None
                    },
                })
            },
        )));

    if let Some(color) = value {
        fields = fields.push(color_picker(
            state.opened_picker == Some(token),
            color,
            button("Pick colour")
                .style(move |theme, status| button::Style {
                    background: Some(iced::Background::Color(color)),
                    ..button::background(theme, status)
                })
                .on_press(Message::Internal(InternalMessage::ShowPicker(Some(token)))),
            Message::Internal(InternalMessage::ShowPicker(None)),
            move |color| {
                Message::Internal(InternalMessage::SetEditorColor {
                    token,
                    color: Some(color),
                })
            },
        ));
    } else {
        fields = fields.push(button("Pick colour").style(button::background));
    }

    Row::new()
        .spacing(4)
        .align_y(Vertical::Center)
        .push(container(label).align_left(Length::Fill))
        .push(fields)
        .into()
}

pub fn update(
    state: &mut State,
    appearance: &mut Appearance,
    message: InternalMessage,
) -> Task<Message> {
    match message {
        InternalMessage::SetTheme(theme) => {
            appearance.theme = theme;
            Task::done(Message::SettingChanged)
        }
        InternalMessage::SetEditorColor { token, color } => {
            let editor = &mut appearance.editor;
            let token = match token {
                EditorToken::Background => &mut editor.background,
                EditorToken::Gridlines => &mut editor.gridlines,
                EditorToken::Glyph => &mut editor.glyph,
                EditorToken::Origin => &mut editor.origin,
                EditorToken::Metrics => &mut editor.metrics,
                EditorToken::Guidelines => &mut editor.guidelines,
            };

            *token = color.map(Into::into);
            state.opened_picker = None;
            Task::done(Message::SettingChanged)
        }
        InternalMessage::ShowPicker(editor_token) => {
            state.opened_picker = editor_token;
            Task::none()
        }
    }
}
