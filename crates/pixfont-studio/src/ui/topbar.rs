// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use iced::{
    Element, Length, Task,
    widget::{Button, Column, Container, Row, Text},
};
use iced_aw::{DropDown, drop_down::Alignment};
use pixfont::export::Exporter;
use pixicons::icon::icon;
use rfd::AsyncFileDialog;

use crate::{
    Message as AMessage, Page,
    project::{self, Project},
};

#[derive(Default)]
pub struct State {
    export_shown: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Open,
    Save,
    SaveAs,
    Export(Exporter),
    ToggleDropdown,
    CloseDropdown,
}

impl State {
    pub fn view(&self, current_page: Page) -> Element<'_, crate::Message> {
        let button_style = |page| {
            if page == current_page {
                iced::widget::button::primary
            } else {
                iced::widget::button::subtle
            }
        };

        Row::new()
            .push(
                Container::new(
                    Row::new()
                        .push(
                            Button::new(icon!(file.font.new))
                                .style(iced::widget::button::subtle)
                                .on_press(project::Action::New.into()),
                        )
                        .push(
                            Button::new(icon!(file))
                                .style(iced::widget::button::subtle)
                                .on_press(AMessage::Topbar(Message::Open)),
                        )
                        .push(
                            Button::new(icon!(save))
                                .style(iced::widget::button::subtle)
                                .on_press(AMessage::Topbar(Message::Save)),
                        )
                        .push(
                            Button::new(icon!(save.new))
                                .style(iced::widget::button::subtle)
                                .on_press(AMessage::Topbar(Message::SaveAs)),
                        )
                        .push(
                            DropDown::new(
                                Button::new(icon!(file.export))
                                    .style(if self.export_shown {
                                        iced::widget::button::primary
                                    } else {
                                        iced::widget::button::subtle
                                    })
                                    .on_press(AMessage::Topbar(Message::ToggleDropdown)),
                                Container::new(Column::with_children(
                                    pixfont::export::EXPORTERS.iter().map(|exporter| {
                                        Button::new(
                                            Row::with_capacity(2)
                                                .push(icon!(file.pentacom))
                                                .push(Text::new(format!("{}", exporter))),
                                        )
                                        .style(iced::widget::button::text)
                                        .width(240)
                                        .on_press(AMessage::Topbar(Message::Export(*exporter)))
                                        .into()
                                    }),
                                ))
                                .style(iced::widget::container::bordered_box),
                                self.export_shown,
                            )
                            .width(Length::Fill)
                            .alignment(Alignment::Bottom)
                            .on_dismiss(AMessage::Topbar(Message::CloseDropdown)),
                        )
                        .spacing(4),
                )
                .align_left(Length::Fill),
            )
            .push(
                Row::new()
                    .push(
                        Button::new("Edit")
                            .style(button_style(Page::Edit))
                            .on_press(AMessage::ShowPage(Page::Edit)),
                    )
                    //.push(
                    //    button("Preview")
                    //        .style(button_style(Page::Edit))
                    //        .on_press(AMessage::ShowPage(Page::Edit)),
                    //)
                    .spacing(4),
            )
            .push(
                Container::new(
                    Row::new()
                        .push(
                            Button::new(icon!(settings))
                                .style(button_style(Page::Settings))
                                .on_press(AMessage::ShowPage(Page::Settings)),
                        )
                        .spacing(4),
                )
                .align_right(Length::Fill),
            )
            .spacing(4)
            .into()
    }

    pub fn update<'a>(
        &'a mut self,
        project: &'a Project,
        message: Message,
    ) -> Task<crate::Message> {
        match message {
            Message::Open => Task::future(async {
                let Some(file) = AsyncFileDialog::new()
                    .add_filter("PixFont Studio Project", &["*.pxfproj"])
                    .add_filter("Pentacom BitFontMaker2", &["*.json"])
                    .pick_file()
                    .await
                else {
                    return crate::Message::None;
                };

                project::Action::Import(file.path().to_path_buf()).into()
            }),

            Message::Save => {
                if project.path.is_some() {
                    Task::done(project::Action::ExportPath.into())
                } else {
                    Self::save_as(project)
                }
            }

            Message::SaveAs => Self::save_as(project),

            Message::Export(exporter) => Task::future(async move {
                let dialog = AsyncFileDialog::new();
                let Some(file) = dialog.pick_file().await else {
                    return crate::Message::Topbar(Message::CloseDropdown);
                };

                project::Action::Export(file.path().to_path_buf(), exporter).into()
            }),

            Message::ToggleDropdown => {
                self.export_shown = !self.export_shown;
                Task::none()
            }

            Message::CloseDropdown => {
                self.export_shown = false;
                Task::none()
            }
        }
    }

    fn save_as(project: &Project) -> Task<crate::Message> {
        let default_filename = project.path.clone().map_or_else(
            || "untitles.pxfproj".to_string(),
            |v| v.to_string_lossy().to_string(),
        );
        Task::future(async {
            let Some(file) = AsyncFileDialog::new()
                .set_file_name(default_filename)
                .add_filter("PixFont Studio Project", &["*.pxfproj"])
                .save_file()
                .await
            else {
                return crate::Message::None;
            };

            project::Action::Export(file.path().to_path_buf(), Exporter::Project).into()
        })
    }
}
