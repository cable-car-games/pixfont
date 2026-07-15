// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use std::{cmp::Ordering, fmt::Display, range::RangeInclusive};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Block {
    pub range: RangeInclusive<u32>,
    pub name: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Codepoint {
    pub codepoint: u32,
    pub name: &'static str,
    pub aliases: &'static [&'static str],
}

include!(concat!(env!("OUT_DIR"), "/ucd.rs"));

impl Block {
    pub fn of(codepoint: u32) -> Option<Block> {
        Self::ALL
            .binary_search_by(|pivot| {
                if codepoint < pivot.range.start {
                    Ordering::Greater
                } else if codepoint > pivot.range.last {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            })
            .ok()
            .map(|index| Self::ALL[index])
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name)
    }
}

impl Codepoint {
    pub fn of(codepoint: u32) -> Option<Codepoint> {
        Self::ALL
            .binary_search_by_key(&codepoint, |pivot| pivot.codepoint)
            .ok()
            .map(|index| Self::ALL[index])
    }
}

#[cfg(test)]
mod tests {
    use crate::{Block, Codepoint};
    use std::assert_matches;

    #[test]
    fn test_block_of() {
        [
            ('A', "Basic Latin"),        // ASCII 'A'
            ('î', "Latin-1 Supplement"), // Circumflex i (the y sound, but in Romanian)
            ('Ş', "Latin Extended-A"),   // Turkish 'sh'
            ('ș', "Latin Extended-B"),   // Romanian 'sh'
        ]
        .into_iter()
        .for_each(|(codepoint, expected)| {
            let Some(Block { name: actual, ..}) = Block::of(codepoint as u32) else {
                panic!("can't find block for '{codepoint}' (U+{:04X})", codepoint as u32)
            };


            assert_eq!(actual, expected, "block for '{codepoint}' (U+{:04X}) is incorrect. wanted \"{expected}\", got \"{actual}\"", codepoint as u32);
        });
    }

    #[test]
    fn test_codepoint_of() {
        [
            ('A', "LATIN CAPITAL LETTER A"),        // ASCII 'A'
            ('î', "LATIN SMALL LETTER I WITH CIRCUMFLEX"), // Circumflex i (the y sound, but in Romanian)
            ('Ş', "LATIN CAPITAL LETTER S WITH CEDILLA"),   // Turkish 'sh'
            ('ș', "LATIN SMALL LETTER S WITH COMMA BELOW"),   // Romanian 'sh'
        ]
        .into_iter()
        .for_each(|(codepoint, expected)| {
            let Some(Codepoint { name: actual, .. }) = Codepoint::of(codepoint as u32) else {
                panic!(
                    "can't find codepoint for '{codepoint}' (U+{:04X})",
                    codepoint as u32
                );
            };

            assert_eq!(actual, expected, "codepoint for '{codepoint}' (U+{:04X}) is incorrect. wanted \"{expected}\", got \"{actual}\"", codepoint as u32)
        });
    }
}
