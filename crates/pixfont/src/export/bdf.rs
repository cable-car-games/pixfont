// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use std::io::Write;

use super::ExportError;
use crate::Font;

pub fn export(_font: &Font, _write: &mut impl Write) -> Result<(), ExportError> {
    todo!();
}
