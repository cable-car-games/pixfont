// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use std::fmt::Display;

use indexmap::IndexSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlyphSet {
    Numbers,
    LatinLower,
    LatinUpper,
    Alphanumeric,
    Punctuation,
    BoxDrawing,
}

pub const DEFINED_GLYPH_SETS: &[GlyphSet] = &[
    GlyphSet::Numbers,
    GlyphSet::LatinLower,
    GlyphSet::LatinUpper,
    GlyphSet::Alphanumeric,
    GlyphSet::Punctuation,
    GlyphSet::BoxDrawing,
];

impl GlyphSet {
    pub fn codepoints(&self) -> IndexSet<u32> {
        match self {
            GlyphSet::Numbers => IndexSet::from_iter(('0' as u32)..=('9' as u32)),
            GlyphSet::LatinLower => IndexSet::from_iter(('a' as u32)..=('z' as u32)),
            GlyphSet::LatinUpper => IndexSet::from_iter(('A' as u32)..=('Z' as u32)),
            GlyphSet::Alphanumeric => [Self::Numbers, Self::LatinLower, Self::LatinUpper]
                .map(|s| s.codepoints())
                .iter()
                .flatten()
                .copied()
                .collect(),
            GlyphSet::Punctuation => todo!(),
            GlyphSet::BoxDrawing => todo!(),
        }
    }
}

impl Display for GlyphSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            GlyphSet::Numbers => "Numbers",
            GlyphSet::LatinLower => "Lowercase letters",
            GlyphSet::LatinUpper => "Uppercase letters",
            GlyphSet::Alphanumeric => "Alphanumeric",
            GlyphSet::Punctuation => "Punctuation",
            GlyphSet::BoxDrawing => "Box drawing characters",
        })
    }
}
