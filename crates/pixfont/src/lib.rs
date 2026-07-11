// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use std::cmp::{max, min};
use std::collections::HashSet;
use std::hash::Hash;

use glifnames::AGLFN;
use glifnames::GlyphName;
use indexmap::IndexMap;

pub mod sets;

pub mod export;
pub mod import;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Default)]
pub struct Font {
    /// Information about the font.
    pub metadata: Metadata,

    /// Font metrics.
    pub metrics: Metrics,

    /// Names and descriptions of alternate mappings.
    pub alternates: IndexMap<String, String>,

    /// Unicode codepoint to character name mappings.
    pub mappings: IndexMap<u32, CodepointMapping>,

    /// Glpyhs.
    pub glyphs: IndexMap<String, Glyph>,
}

impl Font {
    pub fn add_glyph_and_mapping(&mut self, codepoint: u32, name: &str, glyph: Glyph) -> &mut Self {
        self.glyphs.insert(name.to_string(), glyph);
        self.mappings.insert(codepoint, CodepointMapping::new(name));
        self
    }

    pub fn add_glyphs(&mut self, glyphs: &mut impl Iterator<Item = (String, Glyph)>) -> &mut Self {
        for (name, glyph) in glyphs {
            self.glyphs.insert(name, glyph);
        }

        self
    }

    pub fn add_glyphs_and_mappings(
        &mut self,
        glyph_mappings: impl Iterator<Item = (String, u32, Glyph)>,
    ) -> &mut Self {
        for (name, codepoint, glyph) in glyph_mappings {
            self.mappings
                .insert(codepoint, CodepointMapping::new(&name));
            self.glyphs.insert(name, glyph);
        }

        self
    }

    pub fn add_codepoints(&mut self, codepoints: impl Iterator<Item = u32>) -> &mut Self {
        self.add_glyphs_and_mappings(codepoints.map(|codepoint| {
            (
                AGLFN::glyph_name(codepoint).into(),
                codepoint,
                Glyph::default(),
            )
        }))
    }

    pub fn get_glyph_codepoint(&self, glyph_name: &str) -> Option<(u32, Option<&String>)> {
        // FIXME: horrible way to do this, something to optimise later
        //        once the data model has been properly specced

        self.mappings.iter().find_map(|(codepoint, mapping)| {
            if mapping.glyph == glyph_name {
                return Some((*codepoint, None));
            }

            for (alternate, glyph) in &mapping.alternate {
                if glyph == glyph_name {
                    return Some((*codepoint, Some(alternate)));
                }
            }

            None
        })
    }
}

#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Clone, Default)]
pub struct Metrics {
    /// The position where ascenders normally end.
    pub ascender: i32,

    /// The position where descenders normally end.
    pub descender: i32,

    /// The height of capital characters.
    pub cap_height: i32,

    /// The height of lower-case characters (such as 'x').
    pub x_height: i32,

    /// If set, the renderer will attempt to format this font as monospace with
    /// all glyphs advancing this many pixels.
    pub mono_advance: Option<u32>,

    /// Guidelines appearing across all glyphs.
    pub guidelines: Guidelines,
}

impl Metrics {
    pub fn em_size(&self) -> i32 {
        self.ascender - self.descender
    }
}

#[derive(Debug, Clone, Default)]
pub struct CodepointMapping {
    /// Default glyph for this mapping.
    pub glyph: String,

    /// Alternative glyph for this mapping.
    pub alternate: IndexMap<String, String>,
}

impl CodepointMapping {
    pub fn new(glyph: &str) -> Self {
        Self {
            glyph: glyph.into(),
            alternate: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Glyph {
    pub name: String,

    pub pixels: Pixels,

    /// How much space to reserve for this glyph.
    pub advance: u32,

    /// Guidelines for this glyph.
    pub guidelines: Guidelines,

    /// Additional information for exporters.
    pub extra: IndexMap<String, String>,
}

#[derive(Debug, Clone, Default)]
pub struct Pixels {
    pixels: HashSet<Point>,
}

impl Pixels {
    pub fn new() -> Self {
        Self::with_capacity(128)
    }

    pub fn with_capacity(size: usize) -> Self {
        Self {
            pixels: HashSet::with_capacity(size),
        }
    }

    pub fn size(&self) -> Size {
        self.rect().size()
    }

    pub fn rect(&self) -> Rect {
        self.rect_impl().unwrap_or(Rect::ZERO)
    }

    fn rect_impl(&self) -> Option<Rect> {
        let min_x = self.pixels.iter().map(|px| px.x).min()?;
        let min_y = self.pixels.iter().map(|px| px.y).min()?;
        let max_x = self.pixels.iter().map(|px| px.x).max()?;
        let max_y = self.pixels.iter().map(|px| px.y).max()?;

        let bottom_left = Point::new(min_x, min_y);
        let top_right = Point::new(max_x + 1, max_y + 1);

        Some(Rect::with_points(bottom_left, top_right))
    }

    pub fn get(&self, point: Point) -> bool {
        self.pixels.contains(&point)
    }

    pub fn set(&mut self, point: Point, set: bool) -> bool {
        match set {
            true => self.pixels.insert(point),
            false => self.pixels.remove(&point),
        }
    }

    pub fn toggle(&mut self, point: Point) -> bool {
        self.set(point, !self.get(point))
    }

    pub fn pixels(&self) -> impl Iterator<Item = &Point> {
        self.pixels.iter()
    }
}

impl FromIterator<Point> for Pixels {
    fn from_iter<T: IntoIterator<Item = Point>>(iter: T) -> Self {
        Self {
            pixels: HashSet::from_iter(iter),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Guidelines {
    pub x: Vec<Guideline>,
    pub y: Vec<Guideline>,
}

#[derive(Debug, Clone, Default)]
pub struct Guideline {
    pub name: String,
    pub position: i32,
}

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub const ZERO: Size = Size {
        width: 0,
        height: 0,
    };

    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

#[derive(Debug, Clone, Copy, Default, Hash)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub const ZERO: Rect = Rect {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };

    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn with(Point { x, y }: Point, Size { width, height }: Size) -> Self {
        Self::new(x, y, width, height)
    }

    pub fn with_points(p0: Point, p1: Point) -> Self {
        let x0 = min(p0.x, p1.x);
        let y0 = min(p0.y, p1.y);
        let x1 = max(p0.x, p1.x);
        let y1 = max(p0.y, p1.y);

        Self::new(x0, y0, x1.abs_diff(x0), y1.abs_diff(y0))
    }

    pub fn bottom_left(&self) -> Point {
        Point::new(self.x, self.y)
    }

    pub fn left(&self) -> i32 {
        self.x
    }

    pub fn bottom(&self) -> i32 {
        self.y
    }

    pub fn right(&self) -> i32 {
        self.x + (self.width as i32)
    }

    pub fn top(&self) -> i32 {
        self.y + (self.height as i32)
    }

    pub fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }
}
