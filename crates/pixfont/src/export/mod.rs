// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use thiserror::Error;

use crate::Font;

pub mod bdf;
pub mod bmfont;
pub mod pcf;
pub mod pentacom;
pub mod pxfont;
pub mod pxfproj;
pub mod truetype;
pub mod ufo;
pub mod windows;

#[derive(Debug, Clone, Copy)]
pub enum Exporter {
    Binary,
    Project,
    Pentacom,
    BmFont,
    Bdf,
    Pcf,
    Windows,
    Ufo,
    TrueType,
}

impl Display for Exporter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Exporter::Binary => "PixFont binary (.pxf)",
            Exporter::Project => "PixFont Studio project (.pxfproj)",
            Exporter::Pentacom => "Pentacom BitFontMaker2 (.json)",
            Exporter::BmFont => "BMFont PNG Atlas (.fnt)",
            Exporter::Bdf => "X11 / BDF (.bdf)",
            Exporter::Pcf => "X11 / PCF (.pcf)",
            Exporter::Windows => "Windows bitmap font (.fon)",
            Exporter::Ufo => "UFO project",
            Exporter::TrueType => "TrueType (.ttf)",
        })
    }
}

pub const EXPORTERS: &[Exporter] = &[
    //Exporter::Binary,
    Exporter::Pentacom,
    //Exporter::BmFont,
    //Exporter::Bdf,
    //Exporter::Pcf,
    //Exporter::Windows,
    //Exporter::Ufo,
    //Exporter::TrueType,
];

#[derive(Error, Debug)]
pub enum ExportError {
    #[error("format error")]
    Format(#[from] crate::Error),

    #[error("write error")]
    Write(#[from] io::Error),

    #[error("{0}")]
    Message(String),

    #[error("{0}")]
    Misc(Box<dyn Error>),
}

pub fn export(font: &Font, exporter: Exporter, write: &mut impl Write) -> Result<(), ExportError> {
    match exporter {
        Exporter::Binary => pxfont::export(font, write),
        Exporter::Project => pxfproj::export(font, write),
        Exporter::Pentacom => pentacom::export(font, write),
        Exporter::BmFont => bmfont::export(font, write),
        Exporter::Bdf => bdf::export(font, write),
        Exporter::Pcf => pcf::export(font, write),
        Exporter::Windows => windows::export(font, write),
        Exporter::Ufo => ufo::export(font, write),
        Exporter::TrueType => truetype::export(font, write),
    }
}

pub fn export_to_file(font: &Font, exporter: Exporter, path: &Path) -> Result<(), ExportError> {
    let mut file = File::create(path)?;
    export(font, exporter, &mut file)
}
