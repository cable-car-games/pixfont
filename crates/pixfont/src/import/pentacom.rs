// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use std::io::read_to_string;

use glifnames::{AGLFN, GlyphName};
use json::JsonValue::{self};

use crate::{
    Font, Glyph, Guideline, Guidelines, Mapping, Metrics, Pixels, Point, formats::pentacom::*,
    import::ImportError,
};

pub fn import(read: &mut impl std::io::Read) -> Result<Font, ImportError> {
    let json = read_to_string(read)?;
    let json = match json::parse(&json) {
        Ok(json) => json,
        Err(error) => return Err(ImportError::Misc(Box::new(error))),
    };

    let mut font = Font {
        metrics: Metrics {
            ascender: 12,
            descender: -4,
            cap_height: 9,
            x_height: 8,
            guidelines: Guidelines {
                x: vec![Guideline {
                    name: "bearing".into(),
                    position: -2,
                }],
                y: Default::default(),
            },
            ..Default::default()
        },
        ..Default::default()
    };

    let mut warnings: Vec<&'static str> = vec![];

    let mut monospace_flag = false;
    let mut monospace_width = 0u32;

    let mut word_spacing = 5;
    let mut letter_spacing = 1;

    for (key, value) in json.entries() {
        match key {
            "name" => {
                font.metadata.name = match value.as_str() {
                    Some(name) => name.to_string(),
                    None => continue,
                };
            }

            // BitFontMaker2 calls this "AuthorName" in the UI.
            "copy" => {
                font.metadata.author = match value.as_str() {
                    Some(author) => author.to_string(),
                    None => {
                        println!("failed to read 'copy' value as string");
                        continue;
                    }
                };
            }

            // Letter Spacing (0px = 0, 1px = 64, 2px = 128)
            "letterspace" => {
                letter_spacing = match value.as_u32() {
                    Some(value) => value,
                    None => {
                        warnings.push("letterspace: not a u32");
                        continue;
                    }
                };
            }

            // Word spacing (the advance of the space glyph)
            "wordspacing" => {
                word_spacing = match value.as_u32() {
                    Some(value) => value,
                    None => {
                        warnings.push("workspacing: not a u32");
                        continue;
                    }
                }
            }

            "monospace" => {
                monospace_flag = true;
            }

            "monospacewidth" => {
                monospace_width = match value.as_str() {
                    Some(value) => value.parse().unwrap_or_default(),
                    None => 0,
                }
            }

            key => {
                if !try_parse_glyph(key, value, &mut font)? {
                    println!("{key} not handled");
                }
            }
        }
    }

    if monospace_flag && monospace_width > 0 {
        font.metrics.mono_advance = Some(monospace_width);
    }

    if letter_spacing > 0 {
        font.glyphs.iter_mut().for_each(|(_, glyph)| {
            glyph.advance += letter_spacing;
        });
    }

    {
        const SPACE: u32 = ' ' as u32;
        let space_glyph_name = glifnames::AGLFN::glyph_name(SPACE);

        let mapping = font.mappings.entry(' ' as u32).or_insert(Mapping {
            glyph: space_glyph_name.into(),
            alternate: Default::default(),
        });

        font.glyphs.entry(mapping.glyph.clone()).or_insert(Glyph {
            name: mapping.glyph.clone(),
            pixels: Pixels::new(),
            advance: word_spacing,
            guidelines: Default::default(),
            extra: Default::default(),
        })
    };

    Ok(font)
}

fn try_parse_glyph(key: &str, value: &JsonValue, font: &mut Font) -> Result<bool, ImportError> {
    let Ok(codepoint) = key.parse() else {
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
            Some(num) => u16::try_from(num).ok(),
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

    glyph.advance = glyph.pixels.rect().right() as u32 + 1;
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
