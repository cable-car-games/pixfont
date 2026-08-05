// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use crate::{Font, formats::pxfproj::v0, import::ImportError};

pub fn import(read: &mut impl std::io::Read) -> Result<Font, ImportError> {
    let toml = std::io::read_to_string(read)?;
    let file: v0::File =
        toml::from_str(toml.as_str()).map_err(|err| ImportError::Message(err.to_string()))?;
    Ok(file.into())
}
