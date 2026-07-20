// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use std::fmt::Display;

use ::image::RgbaImage;
use iced::{
    Element, Length, Task,
    alignment::Vertical,
    widget::{
        Column, Image, Row, button, column, container,
        image::{FilterMethod, Handle},
        pick_list, row, scrollable, text, text_input,
    },
};
use iced_aw::DropDown;
use image::{Pixel, Rgba};
use itertools::Itertools;
use pixfont::{
    Glyph,
    sets::{DEFINED_GLYPH_SETS, GlyphSet},
};
use pixicons::icon::icon;
use ucd::Codepoint;

use crate::{project, ui::widgets::tooltip::toolbar_tooltip};

pub struct State {
    filter: String,
    sort_by: SortBy,

    results_dirty: bool,
    results: Option<Vec<String>>,

    show_glyphsets: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    SetSortBy(SortBy),
    SetFilter(String),

    ToggleGlyphsetDropdown,
    CloseGlyphsetDropdown,
    AddGlyphSet(GlyphSet),
}

impl From<Message> for crate::Message {
    fn from(message: Message) -> Self {
        self_msg(message)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SortBy {
    None,
    Name,
    #[default]
    Unicode,
}

impl SortBy {
    const ALL: &[SortBy] = &[SortBy::None, SortBy::Name, SortBy::Unicode];
}

impl Display for SortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            SortBy::None => "None",
            SortBy::Name => "Name",
            SortBy::Unicode => "Unicode",
        })
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            filter: Default::default(),
            sort_by: Default::default(),
            results: None,
            results_dirty: true,
            show_glyphsets: false,
        }
    }
}

impl State {
    pub fn view<'state>(
        &'state self,
        font: &'state pixfont::Font,
        selected_glyph: &Option<String>,
    ) -> Element<'state, crate::Message> {
        container(
            Column::new()
                .push(
                    Row::new()
                        .push(button(icon!(new)).style(button::subtle))
                        .push(self.view_new_glyphset())
                        .push(
                            text_input("Search", self.filter.as_str())
                                .on_input(self_msg_fn(Message::SetFilter)),
                        )
                        .push(pick_list(
                            SortBy::ALL,
                            Some(self.sort_by),
                            self_msg_fn(Message::SetSortBy),
                        ))
                        .spacing(4),
                )
                .push(
                    scrollable(container(self.view_glyphs(font, selected_glyph)))
                        .anchor_top()
                        .height(Length::Fill),
                )
                .spacing(4),
        )
        .padding(4)
        .style(container::bordered_box)
        .into()
    }

    fn view_glyphs<'a>(
        &self,
        font: &'a pixfont::Font,
        selected_glyph: &Option<String>,
    ) -> Element<'a, crate::Message> {
        let glyphs: Vec<&'a Glyph> = self.results.as_ref().map_or_else(
            || font.glyphs.iter().map(|(_, glyph)| glyph).collect(),
            |r| {
                r.iter()
                    .map(|name| font.glyphs.get(name).expect("bug: glyph not found"))
                    .collect()
            },
        );

        let glyphs = match self.sort_by {
            SortBy::None => glyphs.into_iter(),
            SortBy::Name => glyphs.into_iter().sorted_by_key(|glyph| &glyph.name),
            SortBy::Unicode => glyphs
                .into_iter()
                .sorted_by_key(|glyph| font.get_glyph_codepoint(&glyph.name)),
        };

        column(glyphs.map(|glyph| {
            let codepoint = font
                .get_glyph_codepoint(&glyph.name)
                .map_or(0xFFFF, |(codepoint, _)| codepoint);
            let codepoint_name =
                Codepoint::of(codepoint).map_or("unknown", |codepoint| codepoint.name);

            button(
                row([
                    Self::glyph_preview(font, glyph),
                    column([
                        (glyph.name.as_str()).into(),
                        text(format!("U+{codepoint:04X} ({codepoint_name})")).into(),
                    ])
                    .into(),
                ])
                .align_y(Vertical::Center)
                .spacing(8),
            )
            .style(if Some(&glyph.name) == selected_glyph.as_ref() {
                button::primary
            } else {
                button::subtle
            })
            .width(Length::Fill)
            .on_press(EditorMessage::SelectGlyph(Some(glyph.name.clone())).into())
            .into()
        }))
        .into()
    }

    fn view_new_glyphset<'a>(&'a self) -> Element<'a, crate::Message> {
        DropDown::new(
            toolbar_tooltip(
                button(icon!(glyphset))
                    .style(button::subtle)
                    .on_press(self_msg(Message::ToggleGlyphsetDropdown)),
                "Add glyphs from a set",
            ),
            container(
                column(DEFINED_GLYPH_SETS.iter().map(|glyphset| {
                    button(row![text(format!("{glyphset}"))])
                        .style(button::subtle)
                        .width(Length::Fill)
                        .on_press(Message::AddGlyphSet(*glyphset).into())
                        .into()
                }))
                .push(
                    button("Unicode block...")
                        .style(button::subtle)
                        .width(Length::Fill),
                ),
            )
            .padding(2)
            .style(container::bordered_box),
            self.show_glyphsets,
        )
        .width(200)
        .on_dismiss(Message::CloseGlyphsetDropdown.into())
        .into()
    }

    fn glyph_preview<'a>(
        font: &'a pixfont::Font,
        glyph: &'a pixfont::Glyph,
    ) -> Element<'a, crate::Message> {
        let min_x: i32 = -2;
        let max_x: i32 = (glyph.advance + 2) as i32;
        let min_y: i32 = font.metrics.descender - 2;
        let max_y: i32 = font.metrics.ascender + 2;

        let mut image = RgbaImage::new(min_x.abs_diff(max_x), min_y.abs_diff(max_y));
        image.fill(255);

        for pixel in glyph.pixels.pixels() {
            if !(min_x..max_x).contains(&pixel.x) || !(min_y..max_y).contains(&pixel.y) {
                continue;
            }

            image.put_pixel(
                (pixel.x - min_x) as u32,
                (max_y - pixel.y - 1) as u32,
                Rgba([0, 0, 0, 255]),
            );
        }

        let handle = Handle::from_rgba(
            image.width(),
            image.height(),
            image
                .pixels()
                .flat_map(|pixel| pixel.channels().iter())
                .copied()
                .collect_vec(),
        );

        Image::new(handle)
            .filter_method(FilterMethod::Nearest)
            .width(64)
            .height(Length::Fill)
            .content_fit(iced::ContentFit::Contain)
            .into()
    }

    pub fn update(&mut self, font: &pixfont::Font, message: Message) -> Task<crate::Message> {
        match message {
            Message::SetSortBy(sort_by) => {
                self.sort_by = sort_by;
                self.results_dirty = true;
            }
            Message::SetFilter(filter) => {
                self.filter = filter;
                self.results_dirty = true;
            }
            Message::ToggleGlyphsetDropdown => {
                self.show_glyphsets = !self.show_glyphsets;
            }
            Message::CloseGlyphsetDropdown => {
                self.show_glyphsets = false;
            }
            Message::AddGlyphSet(glyphset) => {
                self.show_glyphsets = false;
                return Task::done(project::Action::AddGlyphSet(glyphset).into());
            }
        };

        if self.results_dirty {
            self.results = self.update_results(font);
        }

        Task::none()
    }

    fn update_results<'a>(&'a self, font: &'a pixfont::Font) -> Option<Vec<String>> {
        if self.filter.is_empty() {
            return None;
        }

        let candidates = font.glyphs.iter();

        // TODO: apply text filter

        Some(candidates.map(|(name, _)| name).cloned().collect())
    }
}

type InspectorMessage = super::Message;
type EditorMessage = crate::ui::pages::editor::Message;

fn self_msg_fn<'a, A, T>(func: A) -> impl Fn(T) -> crate::Message + 'a
where
    A: (Fn(T) -> Message) + 'a,
    T: 'a,
{
    move |value| self_msg(func(value))
}

fn self_msg(msg: Message) -> crate::Message {
    crate::Message::Editor(EditorMessage::Inspector(InspectorMessage::Directory(msg)))
}
