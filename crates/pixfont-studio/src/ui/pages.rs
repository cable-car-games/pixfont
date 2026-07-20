// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

pub mod editor;
pub mod settings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Edit,
    Settings,
}
