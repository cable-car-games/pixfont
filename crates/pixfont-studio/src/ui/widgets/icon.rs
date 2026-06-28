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
    BiArrowClockwise,
    BiArrowCounterclockwise,
    BiArrowDown,
    BiArrowLeft,
    BiArrowRight,
    BiArrowUp,
    BiArrowsMove,
    BiBorderInner,
    BiClipboard,
    BiCopy,
    BiEraser,
    BiFileArrowDown,
    BiFloppy,
    BiFolder2Open,
    BiGearWideConnected,
    BiPaintBucket,
    BiPen,
    BiPlusLg,
    BiSearch,
    BiSlashLg,
    BiSquare,
    BiZoomIn,
    BiZoomOut,
}

macro_rules! bi {
    ($icon: literal) => {
        include_bytes!(concat!(
            "../../../../../external/bootstrap-icons/icons/",
            $icon,
            ".svg"
        ))
    };
}

impl Icon {
    pub fn svg_data(self) -> &'static [u8] {
        match self {
            Icon::BiArrowClockwise => bi!("arrow-clockwise"),
            Icon::BiArrowCounterclockwise => bi!("arrow-counterclockwise"),
            Icon::BiArrowDown => bi!("arrow-down"),
            Icon::BiArrowLeft => bi!("arrow-left"),
            Icon::BiArrowRight => bi!("arrow-right"),
            Icon::BiArrowUp => bi!("arrow-up"),
            Icon::BiArrowsMove => bi!("arrows-move"),
            Icon::BiBorderInner => bi!("border-inner"),
            Icon::BiClipboard => bi!("clipboard"),
            Icon::BiCopy => bi!("copy"),
            Icon::BiEraser => bi!("eraser"),
            Icon::BiFileArrowDown => bi!("file-arrow-down"),
            Icon::BiFloppy => bi!("floppy"),
            Icon::BiFolder2Open => bi!("folder2-open"),
            Icon::BiGearWideConnected => bi!("gear-wide-connected"),
            Icon::BiPaintBucket => bi!("paint-bucket"),
            Icon::BiPen => bi!("pen"),
            Icon::BiPlusLg => bi!("plus-lg"),
            Icon::BiSearch => bi!("search"),
            Icon::BiSlashLg => bi!("slash-lg"),
            Icon::BiSquare => bi!("square"),
            Icon::BiZoomIn => bi!("zoom-in"),
            Icon::BiZoomOut => bi!("zoom-out"),
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
