// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use std::{ffi::OsStr, path::PathBuf};

use iced::{Element, Font, Pixels, Settings, Task, font, widget::Column};

use crate::ui::{topbar::Topbar, view::directory::Directory};

pub mod ui;

const RAZZA_SANS_REGULAR_TTF: &'static [u8] =
    include_bytes!("ui/font/RazzaSans/Razza Sans Regular.ttf");
const RAZZA_SANS_BOLD_TTF: &'static [u8] = include_bytes!("ui/font/RazzaSans/Razza Sans Bold.ttf");

fn main() -> iced::Result {
    iced::application(Application::boot, Application::update, Application::view)
        .title(Application::title)
        .font(RAZZA_SANS_REGULAR_TTF)
        .font(RAZZA_SANS_BOLD_TTF)
        .settings(Settings {
            default_font: Font {
                family: font::Family::Name("Razza Sans"),
                ..Default::default()
            },
            default_text_size: Pixels::from(13.0),
            ..Default::default()
        })
        .run()
}

struct Application {
    topbar: Topbar,
    directory: Directory,

    dirty: bool,
    project_path: Option<PathBuf>,
    project: Option<pixfont::Font>,
}

#[derive(Debug, Clone)]
enum Message {
    Initialize,
    Topbar(ui::topbar::Message),
    Directory(ui::view::directory::Message),
}

impl Application {
    fn boot() -> (Self, Task<Message>) {
        (
            Self {
                topbar: Topbar::new(),
                directory: Directory::new(),
                dirty: false,
                project_path: None,
                project: None,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Initialize => Task::none(), // TODO: init
            Message::Topbar(message) => match message {
                ui::topbar::Message::NewFile => {
                    // TODO: prompt user to confirm if file is dirty
                    if self.dirty {
                        todo!()
                    }

                    todo!()
                }
                ui::topbar::Message::OpenFile => todo!(),
                ui::topbar::Message::SaveFile => todo!(),
                ui::topbar::Message::ShowView(view) => {
                    self.topbar.view = view;
                    Task::none()
                }
                ui::topbar::Message::OpenExportDropdown(visible) => {
                    self.topbar.export_shown = visible.unwrap_or(!self.topbar.export_shown);
                    Task::none()
                }
                ui::topbar::Message::ExportFile(export_type) => todo!(),
            },
            Message::Directory(message) => match message {
                ui::view::directory::Message::SelectGlyph(_glyph_name) => todo!(),
                _ => self.directory.update(message).map(Message::Directory),
            },
        }
    }

    fn view(&self) -> Element<'_, Message> {
        Column::new()
            .push(self.topbar.view().map(Message::Topbar))
            .push(self.directory.view().map(Message::Directory))
            .padding(8)
            .spacing(8)
            .into()
    }

    fn title(&self) -> String {
        let project_name = match &self.project_path {
            Some(path) => &path
                .file_name()
                .map(OsStr::to_string_lossy)
                .map(|f| f.to_string())
                .unwrap_or("New Project".into()),
            None => "New Project",
        };

        let dirty_mark = if self.dirty { "*" } else { "" };
        format!("{}{} - {}", dirty_mark, project_name, "PixFont Editor")
    }
}
