// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use std::{ffi::OsStr, path::PathBuf};

use iced::{
    Element, Font, Pixels, Settings, Task, font,
    widget::{Column, Text},
};
use rfd::AsyncFileDialog;

use crate::ui::{
    topbar::Topbar,
    view::{directory::Directory, editor::Editor},
};

pub mod ui;

const RAZZA_SANS_REGULAR_TTF: &[u8] = include_bytes!("ui/font/RazzaSans/Razza Sans Regular.ttf");
const RAZZA_SANS_BOLD_TTF: &[u8] = include_bytes!("ui/font/RazzaSans/Razza Sans Bold.ttf");

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
    editor: Editor,

    dirty: bool,
    project_path: Option<PathBuf>,
    project: pixfont::Font,

    selected_glyph: Option<String>,
}

#[derive(Debug, Clone)]
enum Message {
    None,
    OpenProject(Option<PathBuf>, Box<pixfont::Font>),

    Topbar(ui::topbar::Message),
    Directory(ui::view::directory::Message),
    Editor(ui::view::editor::Message),
}

impl Application {
    fn boot() -> (Self, Task<Message>) {
        (
            Self {
                topbar: Default::default(),
                directory: Default::default(),
                editor: Default::default(),
                dirty: false,
                project_path: None,
                project: Default::default(),
                selected_glyph: None,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::None => Task::none(),

            Message::OpenProject(path, font) => {
                self.dirty = false;
                self.project = *font;
                self.project_path = path;
                self.selected_glyph = None;

                Task::none()
            }

            Message::Topbar(message) => match message {
                ui::topbar::Message::NewFile => {
                    // TODO: prompt user to confirm if file is dirty

                    self.dirty = false;
                    self.project = pixfont::Font::default();
                    self.selected_glyph = None;

                    Task::none()
                }
                ui::topbar::Message::OpenFile => Task::future(async {
                    let dialog = AsyncFileDialog::new();
                    let Some(file) = dialog.pick_file().await else {
                        return Message::None;
                    };

                    let font = match pixfont::import::import_from_file(file.path()) {
                        Ok(font) => font,
                        Err(error) => {
                            println!("failed to load file: {}", error);
                            return Message::None;
                        }
                    };

                    Message::OpenProject(Some(file.path().to_owned()), Box::new(font))
                }),
                ui::topbar::Message::SaveFile => {
                    let font = self.project.clone();
                    Task::future(async move {
                        let dialog = AsyncFileDialog::new();
                        let Some(file) = dialog.save_file().await else {
                            return Message::None;
                        };

                        match pixfont::export::export_to_file(
                            &font,
                            pixfont::export::Exporter::Project,
                            file.path(),
                        ) {
                            Ok(_) => {}
                            Err(error) => {
                                println!("failed to save project: {}", error);
                            }
                        }
                        Message::None
                    })
                }
                ui::topbar::Message::ExportFile(exporter) => {
                    let font = self.project.clone();
                    Task::future(async move {
                        let close_msg =
                            Message::Topbar(ui::topbar::Message::OpenExportDropdown(Some(false)));

                        let dialog = AsyncFileDialog::new();
                        let Some(file) = dialog.save_file().await else {
                            return close_msg;
                        };

                        match pixfont::export::export_to_file(&font, exporter, file.path()) {
                            Ok(_) => {}
                            Err(error) => {
                                println!("failed to export file: {}", error);
                            }
                        };
                        close_msg
                    })
                }
                ui::topbar::Message::ShowView(view) => {
                    self.topbar.view = view;
                    Task::none()
                }
                ui::topbar::Message::OpenExportDropdown(visible) => {
                    self.topbar.export_shown = visible.unwrap_or(!self.topbar.export_shown);
                    Task::none()
                }
            },
            Message::Directory(message) => match message {
                ui::view::directory::Message::SelectGlyph(glyph_name) => {
                    self.selected_glyph = Some(glyph_name);
                    Task::none()
                }
                ui::view::directory::Message::AddGlyphsFromSet(set) => {
                    self.project
                        .add_codepoints(&mut set.codepoints().iter().copied());
                    self.directory
                        .update(ui::view::directory::Message::SetDropdown(Some(false)))
                        .map(Message::Directory)
                }

                // metadata update messages
                ui::view::directory::Message::SetName(name) => {
                    self.project.metadata.name = name;
                    Task::none()
                }
                ui::view::directory::Message::SetFamily(family) => {
                    self.project.metadata.family = family;
                    Task::none()
                }
                ui::view::directory::Message::SetWeight(weight) => {
                    self.project.metadata.weight = weight;
                    Task::none()
                }
                ui::view::directory::Message::SetStyle(style) => {
                    self.project.metadata.style = style;
                    Task::none()
                }
                ui::view::directory::Message::SetAuthor(author) => {
                    self.project.metadata.author = author;
                    Task::none()
                }
                ui::view::directory::Message::SetCopyright(copyright) => {
                    self.project.metadata.copyright = copyright;
                    Task::none()
                }
                ui::view::directory::Message::SetLicense(license) => {
                    self.project.metadata.license = license;
                    Task::none()
                }
                ui::view::directory::Message::AddNewExtra => {
                    self.project
                        .metadata
                        .extra
                        .insert("".to_owned(), "".to_owned());
                    Task::none()
                }
                ui::view::directory::Message::SetExtraKey(old, new) => {
                    // TODO: proper error handling
                    //       if the keys in this event are missing, it's likely a bug in some other code
                    self.project
                        .metadata
                        .extra
                        .replace_index(self.project.metadata.extra.get_index_of(&old).unwrap(), new)
                        .unwrap();

                    Task::none()
                }
                ui::view::directory::Message::SetExtraValue(key, value) => {
                    self.project.metadata.extra.insert(key, value);
                    Task::none()
                }
                ui::view::directory::Message::RemoveExtra(key) => {
                    self.project.metadata.extra.shift_remove(&key);
                    Task::none()
                }

                _ => self.directory.update(message).map(Message::Directory),
            },
            Message::Editor(message) => match message {
                ui::view::editor::Message::Private(message) => {
                    self.editor.update(message).map(Message::Editor)
                }
            },
        }
    }

    fn view(&self) -> Element<'_, Message> {
        Column::new()
            .push(self.topbar.view().map(Message::Topbar))
            .push(match self.topbar.view {
                ui::topbar::View::Glyphs => self
                    .directory
                    .view(&self.project, &self.selected_glyph)
                    .map(Message::Directory),
                ui::topbar::View::Edit => self
                    .editor
                    .view(&self.project, &self.selected_glyph)
                    .map(Message::Editor),
                ui::topbar::View::Settings => Text::new("Settings page").into(),
            })
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
        format!("{}{} - {}", dirty_mark, project_name, "PixFont Studio")
    }
}
