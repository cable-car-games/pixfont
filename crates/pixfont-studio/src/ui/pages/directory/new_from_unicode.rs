// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use iced::{
    Element,
    widget::{button, container, pick_list, space},
};
use iced_dialog::dialog;
use ucd::Block;

pub struct NewFromUnicode {
    pub show: bool,
    pub selected: Block,
}

impl Default for NewFromUnicode {
    fn default() -> Self {
        Self {
            show: Default::default(),
            selected: Block::ALL[0],
        }
    }
}

impl NewFromUnicode {
    pub fn view<'a, Message: 'a + Clone>(
        &self,
        base: impl Into<Element<'a, Message>>,
        on_select: impl Fn(Block) -> Message + 'a,
        on_submit: impl Fn(Block) -> Message + 'a,
        on_cancel: Message,
    ) -> Element<'a, Message> {
        dialog(
            self.show,
            base,
            pick_list(Block::ALL, Some(self.selected), on_select),
        )
        .title("New glyph set from Unicode block")
        .push_button(space::horizontal())
        .push_button(
            button("Cancel")
                .style(button::background)
                .on_press(on_cancel.clone()),
        )
        .push_button(button("Add glyphs").on_press(on_submit(self.selected)))
        .container_style(container::bordered_box)
        .on_press(on_cancel.clone())
        .into()
    }
}
