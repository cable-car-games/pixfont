// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use std::io::Write;

use super::ExportError;
use crate::{Font, formats::pxfproj::v0};

pub fn export(font: &Font, write: &mut impl Write) -> Result<(), ExportError> {
    let file = v0::File::from(font.clone());
    let toml = toml::to_string_pretty(&file).unwrap();

    write.write_all(toml.as_bytes())?;
    Ok(())
}
