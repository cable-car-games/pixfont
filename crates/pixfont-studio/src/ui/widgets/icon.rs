// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use iced::{
    Length, Theme,
    widget::{
        Svg,
        svg::{Handle, Style},
    },
};

/// Icons based on SVGs in ../icons.
///
/// Please keep the names as close to 1:1 to the original source.
///
/// - Bi: https://icons.getbootstrap.com/?q=arrow
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Icon {
    BiFloppy,
    BiFolder2Open,
    BiFileArrowDown,
    BiPlusLg,
    BiGearWideConnected,
}

impl Icon {
    pub fn svg_data(self) -> &'static [u8] {
        match self {
            Icon::BiFloppy => {
                include_bytes!("../../../../../external/bootstrap-icons/icons/floppy.svg")
            }
            Icon::BiFolder2Open => {
                include_bytes!("../../../../../external/bootstrap-icons/icons/folder2-open.svg")
            }
            Icon::BiFileArrowDown => {
                include_bytes!("../../../../../external/bootstrap-icons/icons/file-arrow-down.svg")
            }
            Icon::BiPlusLg => {
                include_bytes!("../../../../../external/bootstrap-icons/icons/plus-lg.svg")
            }
            Icon::BiGearWideConnected => {
                include_bytes!(
                    "../../../../../external/bootstrap-icons/icons/gear-wide-connected.svg"
                )
            }
        }
    }

    pub fn as_svg<'a>(self) -> Svg<'a> {
        self.into()
    }
}

impl<'a> From<Icon> for Svg<'a> {
    fn from(icon: Icon) -> Self {
        Svg::new(Handle::from_memory(icon.svg_data()))
            .width(Length::Shrink)
            .style(|theme: &Theme, _status| Style {
                color: Some(theme.palette().text),
            })
            .into()
    }
}
