// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use std::{fmt::Display, range::RangeInclusive};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Block {
    pub range: RangeInclusive<u32>,
    pub name: &'static str,
}

impl Block {
    ucd_macros::blocks! {"ucd/Blocks.txt"}
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name)
    }
}
