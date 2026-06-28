// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use std::io::read_to_string;

use glifnames::{AGLFN, GlyphName};
use json::JsonValue::{self};

use crate::{Font, Glyph, Point, import::ImportError};

pub const GLYPH_HEIGHT: usize = 16;
pub const GLYPH_WIDTH_MAX: usize = 16;

const GLYPH_X_MIN: i32 = -2;
const GLYPH_X_MAX: i32 = 13;
const GLYPH_Y_MIN: i32 = -4;
const GLYPH_Y_MAX: i32 = 11;

pub fn import(read: &mut impl std::io::Read) -> Result<Font, ImportError> {
    let json_string = read_to_string(read)?;
    let json = match json::parse(&json_string) {
        Ok(json) => json,
        Err(error) => return Err(ImportError::Misc(Box::new(error))),
    };

    let mut font = Font::default();

    let mut monospace_flag = false;
    let mut monospace_width = 0u32;

    for (key, value) in json.entries() {
        match key {
            "name" => {
                font.metadata.name = match value.as_str() {
                    Some(name) => name.to_string(),
                    None => String::new(),
                };
            }

            // BitFontMaker2 calls this "AuthorName" in the UI.
            "copy" => {
                font.metadata.author = match value.as_str() {
                    Some(author) => Some(author.to_string()),
                    None => {
                        println!("failed to read 'copy' value as string");
                        None
                    }
                };
            }

            // Letter Spacing (0px = 0, 1px = 64, 2px = 128)
            "letterspace" => {
                // TODO
            }

            // Word spacing (the advance of the space glyph)
            "wordspacing" => {
                // TODO
            }

            "monospace" => {
                monospace_flag = true;
            }
            "monospacewidth" => {
                monospace_width = match value.as_str() {
                    Some(value) => match u32::from_str_radix(value, 10) {
                        Ok(value) => value,
                        Err(_) => 0,
                    },
                    None => 0,
                }
            }

            key => {
                if !try_parse_glyph(key, value, &mut font)? {
                    print!("{key} not handled");
                }
            }
        };
    }

    if monospace_flag && monospace_width > 0 {
        font.metrics.mono_advance = Some(monospace_width);
    }

    Ok(font)
}

fn try_parse_glyph(key: &str, value: &JsonValue, font: &mut Font) -> Result<bool, ImportError> {
    let Ok(codepoint) = u32::from_str_radix(key, 10) else {
        return Ok(false);
    };

    let JsonValue::Array(rows) = value else {
        return Err(ImportError::Message(format!(
            "codepoint {} is not valid",
            codepoint
        )));
    };

    if rows.len() != GLYPH_HEIGHT {
        return Err(ImportError::Message(format!(
            "codepoint {} is not valid",
            codepoint
        )));
    }

    let rows: Vec<Option<u16>> = rows
        .iter()
        .map(|num| match num {
            JsonValue::Number(num) => num.as_fixed_point_u64(0),
            _ => None,
        })
        .map(|num| match num {
            Some(num) => match u16::try_from(num) {
                Ok(num) => Some(num),
                Err(_) => None,
            },
            None => None,
        })
        .collect();

    if rows.iter().any(|f| f.is_none()) {
        return Err(ImportError::Message(format!(
            "codepoint {} is not valid",
            codepoint
        )));
    }

    let mut glyph = Glyph::default();

    for (row_index, row) in rows.iter().map(|s| s.unwrap()).enumerate() {
        let y = GLYPH_Y_MAX - (row_index as i32);

        for x in 0..GLYPH_WIDTH_MAX {
            let x = x as i32;
            let bit = row >> x & 1 != 0;

            glyph.pixels.set(Point::new(GLYPH_X_MIN + x, y), bit);
        }
    }

    font.add_glyph_and_mapping(codepoint, &AGLFN::glyph_name(codepoint), glyph);

    Ok(true)
}

#[cfg(test)]
mod test {
    use std::assert_matches;

    use std::io::Cursor;

    macro_rules! testdata {
        ($path: literal) => {
            include_bytes!(concat!("../../test/pentacom/", $path))
        };
    }

    #[test]
    fn test_born2b_sporty_v2() {
        let data = testdata!("born2b_sporty_v2.json");
        let mut cursor = Cursor::new(data);

        let result = super::import(&mut cursor);
        assert_matches!(result, Ok(_));
    }
}
