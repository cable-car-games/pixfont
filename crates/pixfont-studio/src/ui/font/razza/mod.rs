// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

#![allow(dead_code)]

use iced::{
    Font,
    font::{Family, Stretch, Style, Weight},
};

pub const FAMILY: Family = Family::Name("Razza Sans");

pub const REGULAR_BYTES: &[u8] = include_bytes!("Razza Sans Regular.ttf");
pub const REGULAR: Font = Font {
    family: FAMILY,
    weight: Weight::Normal,
    stretch: Stretch::Normal,
    style: Style::Normal,
};

pub const BOLD_BYTES: &[u8] = include_bytes!("Razza Sans Bold.ttf");
pub const BOLD: Font = Font {
    weight: Weight::Bold,
    ..REGULAR
};

pub const ITALIC_BYTES: &[u8] = include_bytes!("Razza Sans Italic.ttf");
pub const ITALIC: Font = Font {
    style: Style::Italic,
    ..REGULAR
};

pub const BOLD_ITALIC_BYTES: &[u8] = include_bytes!("Razza Sans Bold Italic.ttf");
pub const BOLD_ITALIC: Font = Font {
    style: Style::Italic,
    ..BOLD
};
