// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use std::{fs::File, path::PathBuf};

use iced::Task;
use indexmap::IndexMap;
use pixfont::{
    Font, Guideline, Guidelines,
    export::{ExportError, Exporter},
    import::{ImportError, Importer},
    sets::GlyphSet,
};

pub struct Project {
    pub path: Option<PathBuf>,
    pub font: pixfont::Font,
    dirty: bool,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Action {
    New,
    Import(PathBuf),
    ExportPath,
    Export(PathBuf, Exporter),

    Metadata(SetMetadata),
    Metrics(SetMetrics),
    // TODO: alternates
    Glyph { name: String, action: GlyphAction },

    AddGlyph(AddGlyph),
    AddGlyphs(Vec<AddGlyph>),
    AddGlyphSet(GlyphSet),
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Response {
    Reset,
    Changed,
}

#[derive(Debug, Clone)]
pub enum SetMetadata {
    SetName(String),
    SetFamily(String),
    SetWeight(String),
    SetStyle(String),
    SetAuthor(String),
    SetCopyright(String),
    SetLicense(String),
    Extra(Extra),
}

#[derive(Debug, Clone)]
pub enum Extra {
    Add { key: String, value: String },
    RenameKey { old: String, new: String },
    SetValue { key: String, value: String },
    Remove { key: String },
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum SetMetrics {
    SetAscender(i32),
    SetDescender(i32),
    SetCapHeight(i32),
    SetXHeight(i32),
    SetMonoAdvance(Option<u32>),
    Guideline(Direction, GuidelineAction),
}

#[derive(Debug, Clone)]
pub enum GuidelineAction {
    Create { name: String, position: i32 },
    SetName { index: usize, name: String },
    SetPosition { index: usize, position: i32 },
    SetDirection { index: usize, direction: Direction },
    Remove { index: usize },
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    X,
    Y,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum GlyphAction {
    Rename(String),
    SetMapping(Mapping),
    SetAdvance(u32),
    Guideline(Direction, GuidelineAction),
    Extra(Extra),
}

#[derive(Debug, Clone)]
pub struct AddGlyph {
    pub name: String,
    pub mapping: Option<(u32, Option<String>)>,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Mapping {
    SetCodepoint(u32),
    SetAlternate(Option<String>),
}

#[allow(unused)]
impl GlyphAction {
    pub fn with_glyph(self, name: String) -> Action {
        Action::Glyph { name, action: self }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("no project file path set")]
    NoPath,

    #[error("failed to import project")]
    Import(#[from] ImportError),

    #[error("failed to export project")]
    Export(#[from] ExportError),

    #[error("glyph doesn't exist")]
    GlyphMissing,

    #[error("Glyph already exists")]
    GlyphExists,

    #[error("already contains glyph mapping")]
    AlreadyContainsGlyphMapping,

    #[error("add an alternate mapping with no base glyph mapping")]
    NoBaseGlyphMapping,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            path: Default::default(),
            font: Default::default(),
            dirty: true,
        }
    }
}

impl Project {
    pub fn is_dirty(&self) -> bool {
        self.dirty || self.path.is_none()
    }

    pub fn update(&mut self, message: Action) -> Result<Task<Response>, Error> {
        println!("project update: #{message:#?}");
        let font = &mut self.font;

        self.dirty = true;

        Ok(match message {
            Action::New => {
                *self = Default::default();
                Task::done(Response::Reset)
            }

            Action::Import(path) => {
                let mut file = File::open(&path).map_err(ImportError::Read)?;
                let importer = Importer::from_extension(&path).ok_or(ImportError::NoImporter)?;

                *self = Self {
                    font: pixfont::import::import(importer, &mut file)?,
                    path: Some(path),
                    dirty: false,
                };
                Task::done(Response::Reset)
            }

            Action::ExportPath => {
                let path = self.path.as_ref().ok_or(Error::NoPath)?;

                let mut file = File::create(path).map_err(ExportError::Write)?;
                let exporter = Exporter::Project;
                pixfont::export::export(&self.font, exporter, &mut file)?;

                self.dirty = false;
                Task::none()
            }

            Action::Export(path, exporter) => {
                let mut file = File::create(&path).map_err(ExportError::Write)?;
                pixfont::export::export(&self.font, exporter, &mut file)?;

                if exporter == Exporter::Project {
                    self.path = Some(path);
                    self.dirty = false;
                }

                Task::none()
            }

            Action::Metadata(metadata) => {
                let m = &mut font.metadata;

                match metadata {
                    SetMetadata::SetName(name) => m.name = name,
                    SetMetadata::SetFamily(family) => m.family = family,
                    SetMetadata::SetWeight(weight) => m.weight = weight,
                    SetMetadata::SetStyle(style) => m.style = style,
                    SetMetadata::SetAuthor(author) => m.author = author,
                    SetMetadata::SetCopyright(copyright) => m.copyright = copyright,
                    SetMetadata::SetLicense(license) => m.license = license,
                    SetMetadata::Extra(extra) => Self::update_extra(&mut m.extra, extra),
                };
                Task::none()
            }

            Action::Metrics(metrics) => {
                let m = &mut font.metrics;

                match metrics {
                    SetMetrics::SetAscender(ascender) => m.ascender = ascender,
                    SetMetrics::SetDescender(descender) => m.descender = descender,
                    SetMetrics::SetCapHeight(cap_height) => m.cap_height = cap_height,
                    SetMetrics::SetXHeight(x_height) => m.x_height = x_height,
                    SetMetrics::SetMonoAdvance(mono_advance) => m.mono_advance = mono_advance,
                    SetMetrics::Guideline(direction, action) => {
                        Self::update_guideline(&mut m.guidelines, direction, action)
                    }
                }
                Task::none()
            }

            Action::Glyph { name, action } => {
                let Some(glyph) = font.glyphs.get_mut(&name) else {
                    return Err(Error::GlyphMissing);
                };

                match action {
                    GlyphAction::Rename(_name) => todo!(),
                    GlyphAction::SetMapping(_mapping) => todo!(),
                    GlyphAction::SetAdvance(advance) => glyph.advance = advance,
                    GlyphAction::Extra(action) => Self::update_extra(&mut glyph.extra, action),
                    GlyphAction::Guideline(direction, action) => {
                        Self::update_guideline(&mut glyph.guidelines, direction, action)
                    }
                }

                Task::none()
            }

            Action::AddGlyph(add) => {
                add_glyph(font, add)?;
                Task::none()
            }

            Action::AddGlyphs(adds) => {
                adds.into_iter().try_for_each(|add| add_glyph(font, add))?;
                Task::none()
            }

            Action::AddGlyphSet(glyphset) => {
                font.add_codepoints(glyphset.codepoints().into_iter());
                Task::none()
            }
        })
    }

    fn update_guideline(
        guidelines: &mut Guidelines,
        direction: Direction,
        action: GuidelineAction,
    ) {
        let direction_guidelines = match direction {
            Direction::X => &mut guidelines.x,
            Direction::Y => &mut guidelines.y,
        };

        match action {
            GuidelineAction::Create { name, position } => {
                direction_guidelines.push(Guideline { name, position })
            }

            GuidelineAction::SetName { index, name } => {
                direction_guidelines[index].name = name;
            }

            GuidelineAction::SetPosition { index, position } => {
                direction_guidelines[index].position = position;
            }

            GuidelineAction::Remove { index } => {
                direction_guidelines.remove(index);
            }

            GuidelineAction::SetDirection { index, direction } => {
                let guideline = direction_guidelines.remove(index);
                match direction {
                    Direction::X => &mut guidelines.x,
                    Direction::Y => &mut guidelines.y,
                }
                .push(guideline);
            }
        }
    }

    fn update_extra(map: &mut IndexMap<String, String>, action: Extra) {
        match action {
            Extra::Add { key, value } => {
                map.insert(key, value);
            }

            Extra::RenameKey { old, new } => {
                let index = map.get_index_of(&old).unwrap();
                map.replace_index(index, new).unwrap();
            }

            Extra::SetValue { key, value } => {
                map.insert(key, value);
            }

            Extra::Remove { key } => {
                map.shift_remove(&key);
            }
        }
    }
}

fn add_glyph(font: &mut Font, AddGlyph { name, mapping }: AddGlyph) -> Result<(), Error> {
    if font.glyphs.contains_key(&name) {
        return Err(Error::GlyphExists);
    }

    font.glyphs.insert(
        name.clone(),
        pixfont::Glyph {
            name: name.clone(),
            ..Default::default()
        },
    );

    if let Some((codepoint, alternate)) = mapping {
        if let Some(mapping) = font.mappings.get_mut(&codepoint) {
            if let Some(alternate) = alternate {
                match mapping.alternate.get_mut(&alternate) {
                    Some(_) => Err(Error::AlreadyContainsGlyphMapping),
                    None => {
                        mapping.alternate.insert(alternate, name.clone());
                        Ok(())
                    }
                }
            } else {
                Err(Error::AlreadyContainsGlyphMapping)
            }
        } else {
            if alternate.is_some() {
                Err(Error::NoBaseGlyphMapping)
            } else {
                font.mappings.insert(
                    codepoint,
                    pixfont::Mapping {
                        glyph: name.clone(),
                        alternate: Default::default(),
                    },
                );

                Ok(())
            }
        }
    } else {
        Ok(())
    }
}
