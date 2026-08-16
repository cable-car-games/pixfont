// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use std::ffi::OsStr;

use iced::{Element, Pixels, Settings, Task, Theme, widget::Column, window};

use crate::{project::Project, ui::pages::Page};

mod project;
mod settings;
mod ui;

fn main() -> iced::Result {
    let image =
        image::load_from_memory(include_bytes!("../../../logo.png")).expect("Failed to load icon");
    let icon = iced::window::icon::from_rgba(image.as_bytes().to_vec(), 24, 24).unwrap();

    iced::application(Application::boot, Application::update, Application::view)
        .title(Application::title)
        .theme(Application::theme)
        .settings(Settings {
            default_font: ui::font::razza::REGULAR,
            default_text_size: Pixels::from(13.0),
            fonts: vec![
                ui::font::razza::REGULAR_BYTES.into(),
                ui::font::razza::ITALIC_BYTES.into(),
                ui::font::razza::BOLD_BYTES.into(),
                ui::font::razza::BOLD_ITALIC_BYTES.into(),
            ],
            ..Default::default()
        })
        .window(window::Settings {
            icon: Some(icon),
            ..Default::default()
        })
        .run()
}

struct Application {
    settings: crate::settings::Settings,
    system_theme: iced::theme::Mode,

    project: Project,

    topbar: ui::topbar::State,
    page: Page,
    pages: PageState,
}

struct PageState {
    editor: ui::pages::editor::Editor,
    settings: ui::pages::settings::Settings,
}

#[derive(Debug, Clone)]
enum Message {
    None,

    Edit(project::Action),
    EditCallback(project::Response),

    ShowPage(Page),

    Topbar(ui::topbar::Message),
    Editor(ui::pages::editor::Message),
    Settings(ui::pages::settings::Message),

    SetThemeMode(iced::theme::Mode),
}

impl From<project::Action> for Message {
    fn from(action: project::Action) -> Self {
        Self::Edit(action)
    }
}

impl Application {
    fn boot() -> (Self, Task<Message>) {
        (
            Self {
                // TODO: load settings
                settings: settings::Settings::load().expect("Failed to load settings"),
                system_theme: Default::default(),
                project: Default::default(),
                topbar: Default::default(),
                page: Page::Edit,
                pages: PageState {
                    editor: Default::default(),
                    settings: Default::default(),
                },
            },
            iced::system::theme().map(Message::SetThemeMode),
        )
    }

    fn view(&self) -> Element<'_, Message> {
        Column::new()
            .push(self.topbar.view(self.page))
            .push(match self.page {
                Page::Edit => self.pages.editor.view(&self.settings, &self.project.font),
                Page::Settings => self
                    .pages
                    .settings
                    .view(&self.settings)
                    .map(Message::Settings),
            })
            .padding(8)
            .spacing(8)
            .into()
    }

    fn title(&self) -> String {
        let project_name = match &self.project.path {
            Some(path) => &path
                .file_name()
                .map(OsStr::to_string_lossy)
                .map(|f| f.to_string())
                .unwrap_or("New Project".into()),
            None => "New Project",
        };

        let dirty_mark = if self.project.is_dirty() { "*" } else { "" };
        format!("{}{} - {}", dirty_mark, project_name, "PixFont Studio")
    }

    fn theme(&self) -> Theme {
        match self.settings.appearance.theme {
            settings::Theme::Auto => match self.system_theme {
                iced::theme::Mode::None | iced::theme::Mode::Light => Theme::Light,
                iced::theme::Mode::Dark => Theme::Dark,
            },
            settings::Theme::Light => Theme::Light,
            settings::Theme::Dark => Theme::Dark,
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::None => Task::none(),

            Message::Edit(action) => self.update_project(action),

            Message::EditCallback(response) => match response {
                project::Response::Reset => {
                    self.pages.editor = Default::default();
                    Task::none()
                }
                project::Response::Changed => todo!(),
            },

            Message::ShowPage(page) => {
                self.page = page;
                Task::none()
            }

            Message::Topbar(message) => self.topbar.update(&self.project, message),

            Message::Editor(message) => self.pages.editor.update(message, &mut self.project.font),
            Message::Settings(message) => match message {
                ui::pages::settings::Message::Private(message) => self
                    .pages
                    .settings
                    .update(&mut self.settings, message)
                    .map(Message::Settings),
            },

            Message::SetThemeMode(mode) => {
                self.system_theme = mode;
                Task::none()
            }
        }
    }

    fn update_project(&mut self, action: project::Action) -> Task<Message> {
        let response = match self.project.update(action) {
            Ok(response) => response,
            Err(error) => {
                eprintln!("failed to update project: {error:#?}");
                return Task::none();
            }
        };

        response.map(Message::EditCallback)
    }
}
