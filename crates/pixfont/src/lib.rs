// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use std::cmp::{max, min};

use glifnames::AGLFN;
use glifnames::GlyphName;
use indexmap::IndexMap;

pub mod sets;

#[cfg(test)]
mod tests;

#[derive(Clone, Default)]
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
    pub fn add_glyphs(&mut self, glyphs: &mut impl Iterator<Item = (String, Glyph)>) -> &mut Self {
        for (name, glyph) in glyphs {
            self.glyphs.insert(name, glyph);
        }

        self
    }

    pub fn add_glyphs_and_mappings(
        &mut self,
        glyph_mappings: &mut impl Iterator<Item = (String, u32, Glyph)>,
    ) -> &mut Self {
        for (name, codepoint, glyph) in glyph_mappings {
            self.mappings
                .insert(codepoint, CodepointMapping::new(&name));
            self.glyphs.insert(name, glyph);
        }

        self
    }

    pub fn add_codepoints(&mut self, codepoints: &mut impl Iterator<Item = u32>) -> &mut Self {
        self.add_glyphs_and_mappings(&mut codepoints.map(|codepoint| {
            (
                AGLFN::glyph_name(codepoint).into(),
                codepoint,
                Glyph::default(),
            )
        }))
    }

    pub fn get_glyph_codepoint(&self, glyph_name: &str) -> Option<(u32, Option<&String>)> {
        // FIXME: horrible way to do this, something to optimise later

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

#[derive(Clone, Default)]
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

#[derive(Clone, Default)]
pub struct Metrics {
    /// The position where ascenders normally end.
    pub ascender: i32,

    /// The position where descenders normally end.
    pub descender: i32,

    /// The height of capital characters.
    pub cap_height: i32,

    /// If set, the renderer will attempt to format this font as monospace with
    /// all glyphs advancing this many pixels.
    pub mono_advance: Option<u32>,

    /// Guidelines appearing across all glyphs.
    pub guideline: Guidelines,
}

impl Metrics {
    pub fn em_size(&self) -> i32 {
        self.ascender - self.descender
    }
}

#[derive(Clone, Default)]
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

#[derive(Clone, Default)]
pub struct Glyph {
    pub pixels: Pixels,

    /// How much space to reserve for this glyph.
    pub advance: u32,

    /// Guidelines for this glyph.
    pub guideline: Guidelines,

    /// Additional information for exporters.
    pub extra: IndexMap<String, String>,
}

#[derive(Clone, Default)]
pub struct Pixels {
    pixels: Vec<bool>,
    size: Size,
    origin: Option<Point>,
}

impl Pixels {
    pub fn new() -> Self {
        Self::with_capacity(Size::new(16, 16))
    }

    pub fn with_pixels(pixels: Vec<bool>, size: Size, origin: Point) -> Self {
        Self {
            pixels,
            size,
            origin: Some(origin),
        }
    }

    pub fn with_capacity(size: Size) -> Self {
        let pixel_count = (size.width as usize) * (size.height as usize);

        Self {
            pixels: vec![false; pixel_count],
            size,
            origin: None,
        }
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn origin(&self) -> Point {
        self.origin.unwrap_or_default()
    }

    pub fn rect(&self) -> Rect {
        Rect::with(self.origin(), self.size)
    }

    pub fn pixels(&self) -> Vec<bool> {
        self.pixels.clone()
    }

    fn offset_of(&self, Point { x, y }: Point) -> Option<usize> {
        let origin = self.origin?;

        let x = x.checked_sub(origin.x)? as usize;
        let y = y.checked_sub(origin.y)? as usize;

        let offset = y * (self.size.width as usize) + x;
        Some(offset)
    }

    fn resize_to_include_point(&mut self, point: Point) -> usize {
        if let Some(offset) = self.offset_of(point) {
            return offset;
        }

        todo!()
    }

    pub fn get(&self, point: Point) -> bool {
        if let Some(offset) = self.offset_of(point) {
            self.pixels[offset]
        } else {
            false
        }
    }

    pub fn set(&mut self, point: Point) {
        self.resize_to_include_point(point);
    }
}

#[derive(Clone, Default)]
pub struct Guidelines {
    pub x: Vec<Guideline>,
    pub y: Vec<Guideline>,
}

#[derive(Clone, Default)]
pub struct Guideline {
    pub name: String,
    pub position: i32,
}

#[derive(Clone, Copy, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
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
        let y0 = min(p0.y, p0.y);
        let x1 = max(p0.x, p1.x);
        let y1 = max(p0.y, p1.y);

        Self::new(x0, y0, x1.abs_diff(x0), y1.abs_diff(y0))
    }
}
