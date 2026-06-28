// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use std::cmp::Ordering;

use glam::Vec2;
use iced::{
    Color, Element, Length, Rectangle, Size,
    advanced::{
        Widget, layout,
        renderer::{self, Quad},
    },
};

pub struct GlyphEditor<'state, 'glyph, Message> {
    glyph: &'glyph pixfont::Glyph,
    scale: f32,
    offset: Vec2,
    is_panning: bool,

    on_scale: Option<Box<dyn Fn(f32) -> Message + 'state>>,
    on_pan: Option<Box<dyn Fn(Vec2) -> Message + 'state>>,
}

impl<'state, 'glyph, Message> GlyphEditor<'state, 'glyph, Message> {
    pub fn new(glyph: &'glyph pixfont::Glyph) -> Self {
        GlyphEditor {
            glyph,
            scale: 5.0,
            offset: Vec2::new(0.0, 0.0),
            is_panning: false,

            on_scale: None,
            on_pan: None,
        }
    }

    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    pub fn offset(mut self, offset: Vec2) -> Self {
        self.offset = offset;
        self
    }

    pub fn on_scale(mut self, on_scale: impl Fn(f32) -> Message + 'state) -> Self {
        self.on_scale = Some(Box::new(on_scale));
        self
    }

    pub fn on_pan(mut self, on_pan: impl Fn(Vec2) -> Message + 'state) -> Self {
        self.on_pan = Some(Box::new(on_pan));
        self
    }
}

impl<Message, Theme, Renderer: renderer::Renderer> Widget<Message, Theme, Renderer>
    for GlyphEditor<'_, '_, Message>
{
    fn size(&self) -> iced::Size<iced::Length> {
        Size {
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    fn layout(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        layout::Node::new(limits.max())
    }

    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        let layout_bounds = layout.bounds();

        renderer.fill_quad(
            Quad {
                bounds: layout_bounds,
                ..Default::default()
            },
            Color::from_rgb8(0xFF, 0, 0xFF),
        );

        // gridlines
        let mut gx = layout_bounds.x;
        let mut gy = layout_bounds.y;
        while gx <= layout_bounds.x + layout_bounds.width {
            renderer.fill_quad(
                Quad {
                    bounds: Rectangle {
                        x: gx,
                        y: layout_bounds.y,
                        width: 1.0,
                        height: layout_bounds.height,
                    },
                    ..Default::default()
                },
                Color::from_rgb8(0, 0, 0),
            );

            gx += self.scale;
        }
        while gy <= layout_bounds.y + layout_bounds.height {
            renderer.fill_quad(
                Quad {
                    bounds: Rectangle {
                        x: layout_bounds.x,
                        y: gy,
                        width: layout_bounds.width,
                        height: 1.0,
                    },
                    ..Default::default()
                },
                Color::from_rgb8(0, 0, 0),
            );

            gy += self.scale;
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &iced::advanced::widget::Tree,
        layout: layout::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> iced::advanced::mouse::Interaction {
        let bounds = layout.bounds();

        let Some(cursor_position) = cursor.position() else {
            return iced::advanced::mouse::Interaction::None;
        };

        if !bounds.contains(cursor_position) {
            return iced::advanced::mouse::Interaction::None;
        }

        iced::advanced::mouse::Interaction::Crosshair
    }

    fn update(
        &mut self,
        _tree: &mut iced::advanced::widget::Tree,
        event: &iced::Event,
        _layout: layout::Layout<'_>,
        _cursor: iced::advanced::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        _shell: &mut iced::advanced::Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        println!("{:?}", event);
        match event {
            iced::Event::Keyboard(event) => {}
            iced::Event::Mouse(event) => match event {
                iced::mouse::Event::CursorEntered => {}
                iced::mouse::Event::CursorLeft => {}
                iced::mouse::Event::CursorMoved { position } => {}
                iced::mouse::Event::ButtonPressed(button) => {
                    // TODO: dispatch based on what needs to be done
                }
                iced::mouse::Event::ButtonReleased(button) => {
                    // TODO: dispatch based on what needs to be done
                }
                iced::mouse::Event::WheelScrolled { delta } => {
                    let y = match delta {
                        iced::mouse::ScrollDelta::Lines { x: _x, y } => *y,
                        iced::mouse::ScrollDelta::Pixels { x: _x, y } => *y,
                    };

                    let message = if let Some(on_scale) = &self.on_scale {
                        println!("try resize");
                        match y.total_cmp(&0.0) {
                            Ordering::Less => Some(on_scale(f32::max(2.0, self.scale * 0.9))),
                            Ordering::Greater => Some(on_scale(self.scale * 1.1)),
                            _ => None,
                        }
                    } else {
                        None
                    };

                    if let Some(message) = message {
                        if _cursor.is_over(_layout.bounds()) {
                            _shell.publish(message);
                        }
                        _shell.capture_event();
                    }
                }
            },
            iced::Event::Window(event) => {}
            iced::Event::Touch(event) => {}
            iced::Event::InputMethod(event) => {}
        }
    }
}

impl<'state, 'glyph: 'state, Message: 'state, Theme, Renderer: renderer::Renderer>
    From<GlyphEditor<'state, 'glyph, Message>> for Element<'state, Message, Theme, Renderer>
{
    fn from(widget: GlyphEditor<'state, 'glyph, Message>) -> Self {
        Self::new(widget)
    }
}
