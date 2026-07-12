// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use std::{
    error::Error,
    ffi::OsStr,
    fmt::Display,
    fs::File,
    io::{self, Read},
    path::Path,
};

use thiserror::Error;

use crate::Font;

pub mod pxfont;
pub mod pxfproj;

pub mod pentacom;

#[derive(Clone, Copy)]
pub enum Importer {
    Binary,
    Project,
    Pentacom,
}

pub const IMPORTERS: &[Importer] = &[Importer::Binary, Importer::Project, Importer::Pentacom];

// TODO: the way we do imports need to change so we can determine the format
//       from the file contents

impl Importer {
    pub fn from_extension(path: &Path) -> Option<Self> {
        let extension = path.extension()?;

        Some(
            if extension == OsStr::new("pixfont") || extension == OsStr::new("pxf") {
                Importer::Binary
            } else if extension == OsStr::new("pxproj") || extension == OsStr::new("pxfproj") {
                Importer::Project
            } else if extension == OsStr::new("json")
                || extension == OsStr::new("txt")
                || extension == OsStr::new("pentacom")
            {
                Importer::Pentacom
            } else {
                return None;
            },
        )
    }
}

impl Display for Importer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Importer::Binary => "PixFont Font",
            Importer::Project => "PixFont Studio Project",
            Importer::Pentacom => "Pentacom",
        })
    }
}

// TODO: we also need to split up import errors into two categories:
//       - bad format (try the next one)
//       - crash out (can't read the file)
#[derive(Error, Debug)]
pub enum ImportError {
    #[error("read error")]
    Read(#[from] io::Error),

    #[error("no importer for file")]
    NoImporter,

    #[error("{0}")]
    Message(String),

    #[error("{0}")]
    Misc(Box<dyn Error>),
}

pub fn import(importer: Importer, read: &mut impl Read) -> Result<Font, ImportError> {
    match importer {
        Importer::Binary => todo!(),
        Importer::Project => pxfproj::import(read),
        Importer::Pentacom => pentacom::import(read),
    }
}

pub fn import_from_file(path: &Path) -> Result<Font, ImportError> {
    let importer = match Importer::from_extension(path) {
        Some(importer) => importer,
        None => return Err(ImportError::NoImporter),
    };

    let mut file = File::open(path)?;
    import(importer, &mut file)
}
