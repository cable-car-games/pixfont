// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use iced::{
    Element, padding,
    widget::{Tooltip, container, tooltip::Position},
};

pub fn toolbar_tooltip<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    tooltip: impl Into<Element<'a, Message>>,
) -> Tooltip<'a, Message> {
    Tooltip::new(
        content,
        container(tooltip)
            .style(container::bordered_box)
            .padding(padding::vertical(4).horizontal(8)),
        Position::Bottom,
    )
}
