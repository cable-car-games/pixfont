// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::Point;

pub const VERSION: &str = "dev";
pub const EXPORTER: &str = "PixFont Studio dev build";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub version: String,
    pub exporter: Option<String>,
    pub metadata: Metadata,
    pub metrics: Metrics,
    pub alternates: IndexMap<String, String>,
    pub mappings: IndexMap<u32, Mapping>,
    pub glyphs: IndexMap<String, Glyph>,
}

impl From<crate::Font> for File {
    fn from(font: crate::Font) -> Self {
        Self {
            version: String::from(VERSION),
            exporter: Some(String::from(EXPORTER)),
            metadata: font.metadata.into(),
            metrics: font.metrics.into(),
            alternates: font.alternates,
            mappings: font
                .mappings
                .into_iter()
                .map(|(key, value)| (key, value.into()))
                .collect(),
            glyphs: font
                .glyphs
                .into_iter()
                .map(|(key, value)| (key, value.into()))
                .collect(),
        }
    }
}

impl From<File> for crate::Font {
    fn from(file: File) -> Self {
        Self {
            metadata: file.metadata.into(),
            metrics: file.metrics.into(),
            alternates: file.alternates.into(),
            mappings: file
                .mappings
                .into_iter()
                .map(|(key, value)| (key, value.into()))
                .collect(),
            glyphs: file
                .glyphs
                .into_iter()
                .map(|glyph| {
                    let (key, ..) = &glyph;
                    (key.clone(), glyph.into())
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub family: Option<String>,
    pub weight: Option<String>,
    pub style: Option<String>,
    pub author: Option<String>,
    pub copyright: Option<String>,
    pub license: Option<String>,
    pub extra: IndexMap<String, String>,
}

impl From<crate::Metadata> for Metadata {
    fn from(
        crate::Metadata {
            name,
            family,
            weight,
            style,
            author,
            copyright,
            license,
            extra,
        }: crate::Metadata,
    ) -> Self {
        Self {
            name,
            family,
            weight,
            style,
            author,
            copyright,
            license,
            extra,
        }
    }
}

impl From<Metadata> for crate::Metadata {
    fn from(
        Metadata {
            name,
            family,
            weight,
            style,
            author,
            copyright,
            license,
            extra,
        }: Metadata,
    ) -> Self {
        Self {
            name,
            family,
            weight,
            style,
            author,
            copyright,
            license,
            extra,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metrics {
    pub ascender: i32,
    pub descender: i32,
    pub cap_height: i32,
    pub x_height: i32,
    pub mono_advance: Option<u32>,
    pub guidelines: Guidelines,
}

impl From<crate::Metrics> for Metrics {
    fn from(
        crate::Metrics {
            ascender,
            descender,
            cap_height,
            x_height,
            mono_advance,
            guidelines,
        }: crate::Metrics,
    ) -> Self {
        Self {
            ascender,
            descender,
            cap_height,
            x_height,
            mono_advance,
            guidelines: guidelines.into(),
        }
    }
}

impl From<Metrics> for crate::Metrics {
    fn from(
        Metrics {
            ascender,
            descender,
            cap_height,
            x_height,
            mono_advance,
            guidelines,
        }: Metrics,
    ) -> Self {
        Self {
            ascender,
            descender,
            cap_height,
            x_height,
            mono_advance,
            guidelines: guidelines.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Mapping {
    Single(String),
    Multiple {
        glyph: String,
        alternate: IndexMap<String, String>,
    },
}

impl From<crate::CodepointMapping> for Mapping {
    fn from(crate::CodepointMapping { glyph, alternate }: crate::CodepointMapping) -> Self {
        if alternate.is_empty() {
            Self::Single(glyph)
        } else {
            Self::Multiple { glyph, alternate }
        }
    }
}

impl From<Mapping> for crate::CodepointMapping {
    fn from(mapping: Mapping) -> Self {
        match mapping {
            Mapping::Single(glyph) => Self {
                glyph,
                ..Default::default()
            },
            Mapping::Multiple { glyph, alternate } => Self { glyph, alternate },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Glyph {
    pixels: Pixels,
    advance: u32,
    guidelines: Guidelines,
    extra: IndexMap<String, String>,
}

impl From<crate::Glyph> for Glyph {
    fn from(
        crate::Glyph {
            name: _,
            pixels,
            advance,
            guidelines,
            extra,
        }: crate::Glyph,
    ) -> Self {
        Self {
            pixels: pixels.into(),
            advance,
            guidelines: guidelines.into(),
            extra,
        }
    }
}

impl From<(String, Glyph)> for crate::Glyph {
    fn from(
        (
            name,
            Glyph {
                pixels,
                advance,
                guidelines,
                extra,
            },
        ): (String, Glyph),
    ) -> Self {
        Self {
            name,
            pixels: pixels.into(),
            advance,
            guidelines: guidelines.into(),
            extra,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pixels {
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
    pub pixels: String,
}

impl Pixels {
    const ON: char = 'X';
    const OFF: char = '.';

    pub fn width(&self) -> u32 {
        self.max_x.abs_diff(self.min_x) + 1
    }

    pub fn height(&self) -> u32 {
        self.max_y.abs_diff(self.min_y) + 1
    }
}

impl From<crate::Pixels> for Pixels {
    fn from(pixels: crate::Pixels) -> Self {
        let rect = pixels.rect();
        let min_x = rect.left();
        let min_y = rect.bottom();
        let max_x = rect.right();
        let max_y = rect.top();

        let pixels = {
            let mut repr = String::new();

            for row in (min_y..=max_y).rev() {
                for col in min_x..=max_x {
                    repr.push(if pixels.get(Point::new(col, row)) {
                        Self::ON
                    } else {
                        Self::OFF
                    });
                }
                repr.push('\n');
            }

            repr
        };

        Self {
            min_x,
            min_y,
            max_x,
            max_y,
            pixels,
        }
    }
}

impl From<Pixels> for crate::Pixels {
    fn from(pixels: Pixels) -> Self {
        let mut out = Self::with_capacity(pixels.pixels.len());

        let mut row = pixels.max_y;
        let mut col = pixels.min_x;

        let pixel_count = pixels
            .pixels
            .chars()
            .filter(|c| *c == Pixels::ON || *c == Pixels::OFF)
            .count();
        let wanted_pixel_count = (pixels.width() * pixels.height()) as usize;

        if pixel_count != wanted_pixel_count {
            panic!(
                "pixel count mismatch: wanted {}, got {}",
                wanted_pixel_count, pixel_count
            );
        }

        for c in pixels.pixels.chars() {
            match c {
                Pixels::ON | Pixels::OFF => {
                    out.set(Point::new(col, row), c == Pixels::ON);
                }
                _ => continue,
            };

            col += 1;

            if col > pixels.max_x {
                col = 0;
                row -= 1;
            }
        }

        out
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guidelines {
    pub x: Vec<Guideline>,
    pub y: Vec<Guideline>,
}

impl From<crate::Guidelines> for Guidelines {
    fn from(crate::Guidelines { x, y }: crate::Guidelines) -> Self {
        Self {
            x: x.into_iter().map(Into::into).collect(),
            y: y.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<Guidelines> for crate::Guidelines {
    fn from(Guidelines { x, y }: Guidelines) -> Self {
        Self {
            x: x.into_iter().map(Into::into).collect(),
            y: y.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guideline {
    pub name: String,
    pub position: i32,
}

impl From<crate::Guideline> for Guideline {
    fn from(crate::Guideline { name, position }: crate::Guideline) -> Self {
        Self { name, position }
    }
}

impl From<Guideline> for crate::Guideline {
    fn from(Guideline { name, position }: Guideline) -> Self {
        Self { name, position }
    }
}
