// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use std::io::Write;

use json::JsonValue;

use super::ExportError;
use crate::formats::pentacom::*;
use crate::{Error, Font, Point};

pub fn export(font: &Font, writer: &mut impl Write) -> Result<(), ExportError> {
    let mut json = json::object! {
        name: font.metadata.name.as_str(),
    };

    if !font.metadata.author.is_empty() {
        json["copy"] = font.metadata.author.as_str().into();
    }

    if let Some(mono_advance) = font.metrics.mono_advance {
        json["monospace"] = true.into();
        json["monospacewidth"] = mono_advance.into();
    }

    if let Some(mapping) = font.mappings.get(&(' ' as u32)) {
        let glyph = font
            .glyphs
            .get(&mapping.glyph)
            .expect("glyph doesn't exist");
        json["wordspacing"] = glyph.advance.into();
    }

    for (codepoint, mapping) in &font.mappings {
        let codepoint = *codepoint;

        // ignore spaces, handled separately
        if codepoint == ' ' as u32 {
            continue;
        }

        let glyph = font
            .glyphs
            .get(&mapping.glyph)
            .ok_or_else(|| Error::GlyphNotFound {
                name: mapping.glyph.clone(),
            })?;

        let mut out = Vec::with_capacity(16);

        for row in (GLYPH_Y_MIN..=GLYPH_Y_MAX).rev() {
            let mut out_row = 0u16;
            for col in (GLYPH_X_MIN..=GLYPH_X_MAX).rev() {
                let on = if glyph.pixels.get(Point::new(col, row)) {
                    1
                } else {
                    0
                };

                out_row = out_row << 1 | on;
            }
            out.push(out_row);
        }

        json[&codepoint.to_string()] = JsonValue::from(out);
    }

    writer.write_all(&json.to_string().into_bytes())?;
    Ok(())
}
