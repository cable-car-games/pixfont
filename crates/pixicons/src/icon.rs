// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use iced::{Element, widget::image::FilterMethod};

use crate::Layers;

use pixicons_macros;
pub use pixicons_macros::icon;

pub struct Icon<'a> {
    layers: Layers<'a>,
}

impl<'a> Icon<'a> {
    pub fn new(layers: impl Into<Layers<'a>>) -> Self {
        Self {
            layers: layers.into(),
        }
    }
}

impl<'a, Message: 'a> From<Icon<'a>> for Element<'a, Message> {
    fn from(icon: Icon<'a>) -> Self {
        iced::widget::image(icon.layers)
            .width(16)
            .height(16)
            .filter_method(FilterMethod::Nearest)
            .into()
    }
}
