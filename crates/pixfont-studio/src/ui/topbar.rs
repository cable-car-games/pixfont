// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use std::fmt::Display;

use iced::{
    Element, Length,
    widget::{Button, Column, Container, Row, Text},
};
use iced_aw::{DropDown, drop_down::Alignment};

use crate::ui::widgets::icon::Icon;

pub struct Topbar {
    pub view: View,
    pub export_shown: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    NewFile,
    OpenFile,
    SaveFile,
    ShowView(View),

    OpenExportDropdown(Option<bool>),
    ExportFile(ExportType),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum View {
    Glyphs,
    Edit,
    Settings,
}

// TODO: move to core module when export is implemented
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportType {
    /// Pentacom BitFontMaker2 JSON
    ///
    /// https://www.pentacom.jp/pentacom/bitfontmaker2/
    Pentacom,

    /// Adobe Glyph Bitmap Distribution Format (BDF)
    Bdf,

    /// Portable Compiled Format
    ///
    /// https://fontforge.org/docs/techref/pcf-format.html
    Pcf,

    /// FontForge project
    ///
    /// https://fontforge.org/
    FontForge,

    /// TrueType format
    ///
    /// - https://en.wikipedia.org/wiki/TrueType
    /// - https://developer.apple.com/fonts/TrueType-Reference-Manual/
    Ttf,

    /// Windows and OS/2 bitmap font format
    ///
    /// - https://web.archive.org/web/20080115184921/http://support.microsoft.com/kb/65123
    /// - https://web.archive.org/web/20120312000908/http://www.csn.ul.ie/%7Ecaolan/publink/winresdump/winresdump/doc/resfmt.txt
    Fon,

    /// AngelCode BMFont format (PNG + altas)
    ///
    /// - https://angelcode.com/products/bmfont/
    Bmf,
}

impl Display for ExportType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ExportType::Pentacom => "Pentacom BitFontMaker2™ (.json)",
            ExportType::Bdf => "BDF (.bdf)",
            ExportType::Pcf => "X11/PCF (.pcf)",
            ExportType::FontForge => "FontForge project (.zip)",
            ExportType::Ttf => "TrueType (.ttf)",
            ExportType::Fon => "Windows bitmap font (.fon)",
            ExportType::Bmf => "AngelCode BMFont (.fnt + .json)",
        })
    }
}

impl Topbar {
    pub fn new() -> Self {
        Self {
            view: View::Glyphs,
            export_shown: false,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let button_style = |view: View| {
            if view == self.view {
                iced::widget::button::primary
            } else {
                iced::widget::button::subtle
            }
        };

        Row::new()
            .push(
                Container::new(
                    Row::new()
                        .push(
                            Button::new(Icon::BiPlusLg.as_svg())
                                .style(iced::widget::button::subtle)
                                .on_press(Message::NewFile),
                        )
                        .push(
                            Button::new(Icon::BiFolder2Open.as_svg())
                                .style(iced::widget::button::subtle)
                                .on_press(Message::OpenFile),
                        )
                        .push(
                            Button::new(Icon::BiFloppy.as_svg())
                                .style(iced::widget::button::subtle)
                                .on_press(Message::SaveFile),
                        )
                        .push(
                            DropDown::new(
                                Button::new(Icon::BiFileArrowDown.as_svg())
                                    .style(if self.export_shown {
                                        iced::widget::button::primary
                                    } else {
                                        iced::widget::button::subtle
                                    })
                                    .on_press(Message::OpenExportDropdown(None)),
                                Container::new(Column::with_children(
                                    [
                                        ExportType::Bdf,
                                        ExportType::Pcf,
                                        ExportType::Ttf,
                                        ExportType::Fon,
                                        ExportType::Bmf,
                                        ExportType::Pentacom,
                                    ]
                                    .map(|export_type| {
                                        Button::new(Text::new(format!("{}", export_type)))
                                            .style(iced::widget::button::text)
                                            .width(240)
                                            .on_press(Message::ExportFile(export_type))
                                            .into()
                                    }),
                                ))
                                .style(iced::widget::container::bordered_box),
                                self.export_shown,
                            )
                            .width(Length::Fill)
                            .alignment(Alignment::Bottom)
                            .on_dismiss(Message::OpenExportDropdown(Some(false))),
                        )
                        .spacing(4),
                )
                .align_left(Length::Fill),
            )
            .push(
                Row::new()
                    .push(
                        Button::new("Glyphs")
                            .style(button_style(View::Glyphs))
                            .on_press(Message::ShowView(View::Glyphs)),
                    )
                    .push(
                        Button::new("Edit")
                            .style(button_style(View::Edit))
                            .on_press(Message::ShowView(View::Edit)),
                    )
                    .spacing(4),
            )
            .push(
                Container::new(
                    Row::new()
                        .push(
                            Button::new(Icon::BiGearWideConnected.as_svg())
                                .style(button_style(View::Settings))
                                .on_press(Message::ShowView(View::Settings)),
                        )
                        .spacing(4),
                )
                .align_right(Length::Fill),
            )
            .spacing(4)
            .into()
    }
}
