// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use std::{ffi::OsStr, path::PathBuf};

use iced::{Element, Pixels, Settings, Task, widget::Column, window};
use rfd::AsyncFileDialog;

pub mod ui;

const RAZZA_SANS_REGULAR_TTF: &[u8] = include_bytes!("ui/font/RazzaSans/Razza Sans Regular.ttf");
const RAZZA_SANS_BOLD_TTF: &[u8] = include_bytes!("ui/font/RazzaSans/Razza Sans Bold.ttf");

fn main() -> iced::Result {
    let image =
        image::load_from_memory(include_bytes!("../../../logo.png")).expect("Failed to load icon");
    let icon = iced::window::icon::from_rgba(image.as_bytes().to_vec(), 24, 24).unwrap();

    iced::application(Application::boot, Application::update, Application::view)
        .title(Application::title)
        .font(RAZZA_SANS_REGULAR_TTF)
        .font(RAZZA_SANS_BOLD_TTF)
        .settings(Settings {
            // default_font: Font {
            //     family: font::Family::Name("Razza Sans"),
            //     ..Default::default()
            // },
            default_text_size: Pixels::from(13.0),
            ..Default::default()
        })
        .window(window::Settings {
            icon: Some(icon),
            ..Default::default()
        })
        .run()
}

struct Application {
    topbar: ui::topbar::Topbar,
    directory: ui::view::directory::Directory,
    editor: ui::view::editor::Editor,
    settings: ui::view::settings::Settings,

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
    Settings(ui::view::settings::Message),
}

impl Application {
    fn boot() -> (Self, Task<Message>) {
        (
            Self {
                topbar: Default::default(),
                directory: Default::default(),
                editor: Default::default(),
                settings: Default::default(),
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
                ui::view::editor::Message::SetGlyphProp(glyph_prop) => {
                    let glyph_name = self
                        .selected_glyph
                        .as_ref()
                        .expect("bug: no selected glyph");

                    let glyph = self
                        .project
                        .glyphs
                        .get_mut(glyph_name)
                        .expect("bug: selected glyph does not exist");

                    match glyph_prop {
                        ui::view::editor::GlyphProp::Advance(value) => glyph.advance = value,

                        // TODO: the ones below should probably be moved elsewhere
                        ui::view::editor::GlyphProp::Ascender(value) => {
                            self.project.metrics.ascender = value
                        }
                        ui::view::editor::GlyphProp::Descender(value) => {
                            self.project.metrics.descender = value
                        }
                        ui::view::editor::GlyphProp::CapHeight(value) => {
                            self.project.metrics.cap_height = value
                        }
                        ui::view::editor::GlyphProp::XHeight(value) => {
                            self.project.metrics.x_height = value
                        }
                    };
                    Task::none()
                }
                ui::view::editor::Message::SetGuideline(guideline_action) => match guideline_action
                {
                    ui::view::editor::GuidelineAction::Create => {
                        let glyph_name = self
                            .selected_glyph
                            .as_ref()
                            .expect("bug: no selected glyph");
                        let glyph = self
                            .project
                            .glyphs
                            .get_mut(glyph_name)
                            .expect("bug: selected glyph does not exist");

                        glyph.guidelines.x.push(pixfont::Guideline {
                            name: "".to_string(),
                            position: 16,
                        });

                        Task::none()
                    }
                    ui::view::editor::GuidelineAction::Rename {
                        scope,
                        direction,
                        index,
                        name,
                    } => {
                        let scope = match scope {
                            ui::view::editor::GuidelineScope::Global => {
                                &mut self.project.metrics.guidelines
                            }
                            ui::view::editor::GuidelineScope::Local => {
                                let glyph_name = self
                                    .selected_glyph
                                    .as_ref()
                                    .expect("bug: no selected glyph");
                                let glyph = self
                                    .project
                                    .glyphs
                                    .get_mut(glyph_name)
                                    .expect("bug: selected glyph does not exist");

                                &mut glyph.guidelines
                            }
                        };

                        let direction = match direction {
                            ui::view::editor::GuidelineDirection::X => &mut scope.x,
                            ui::view::editor::GuidelineDirection::Y => &mut scope.y,
                        };

                        let guideline =
                            direction.get_mut(index).expect("bug: index does not exist");

                        guideline.name = name;
                        Task::none()
                    }
                    ui::view::editor::GuidelineAction::Set {
                        scope,
                        direction,
                        index,
                        position,
                    } => {
                        let scope = match scope {
                            ui::view::editor::GuidelineScope::Global => {
                                &mut self.project.metrics.guidelines
                            }
                            ui::view::editor::GuidelineScope::Local => {
                                let glyph_name = self
                                    .selected_glyph
                                    .as_ref()
                                    .expect("bug: no selected glyph");
                                let glyph = self
                                    .project
                                    .glyphs
                                    .get_mut(glyph_name)
                                    .expect("bug: selected glyph does not exist");

                                &mut glyph.guidelines
                            }
                        };

                        let direction = match direction {
                            ui::view::editor::GuidelineDirection::X => &mut scope.x,
                            ui::view::editor::GuidelineDirection::Y => &mut scope.y,
                        };

                        let guideline =
                            direction.get_mut(index).expect("bug: index does not exist");

                        guideline.position = position;
                        Task::none()
                    }
                    ui::view::editor::GuidelineAction::Remove {
                        scope: _,
                        direction: _,
                        index: _name,
                    } => todo!(),
                    ui::view::editor::GuidelineAction::MakeGlobal {
                        direction: _,
                        index: _,
                    } => {
                        todo!();
                        // Task::none()
                    }
                    ui::view::editor::GuidelineAction::MakeLocal {
                        direction: _,
                        index: _,
                    } => {
                        todo!();
                        // Task::none()
                    }
                    ui::view::editor::GuidelineAction::MakeX { scope, index } => {
                        let scope = match scope {
                            ui::view::editor::GuidelineScope::Global => {
                                &mut self.project.metrics.guidelines
                            }
                            ui::view::editor::GuidelineScope::Local => {
                                let glyph_name = self
                                    .selected_glyph
                                    .as_ref()
                                    .expect("bug: no selected glyph");
                                let glyph = self
                                    .project
                                    .glyphs
                                    .get_mut(glyph_name)
                                    .expect("bug: selected glyph does not exist");

                                &mut glyph.guidelines
                            }
                        };

                        scope.x.push(scope.y.remove(index));
                        Task::none()
                    }
                    ui::view::editor::GuidelineAction::MakeY { scope, index } => {
                        let scope = match scope {
                            ui::view::editor::GuidelineScope::Global => {
                                &mut self.project.metrics.guidelines
                            }
                            ui::view::editor::GuidelineScope::Local => {
                                let glyph_name = self
                                    .selected_glyph
                                    .as_ref()
                                    .expect("bug: no selected glyph");
                                let glyph = self
                                    .project
                                    .glyphs
                                    .get_mut(glyph_name)
                                    .expect("bug: selected glyph does not exist");

                                &mut glyph.guidelines
                            }
                        };

                        scope.y.push(scope.x.remove(index));
                        Task::none()
                    }
                },
            },
            Message::Settings(message) => match message {
                ui::view::settings::Message::Private(message) => {
                    self.settings.update(message).map(Message::Settings)
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
                    .view(&self.project, self.selected_glyph.as_ref())
                    .map(Message::Editor),
                ui::topbar::View::Settings => self.settings.view().map(Message::Settings),
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
