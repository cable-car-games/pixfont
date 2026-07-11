// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use std::{cmp::Ordering, collections::HashSet};

use iced::{
    Background, Color, Element, Length, Point, Rectangle, Size, Vector,
    advanced::{
        Widget, layout,
        renderer::{self, Quad},
    },
    keyboard::{Key, key},
};

use crate::settings::{Appearance, EditorColors};

pub struct GlyphEditor<'state, 'glyph, Message> {
    glyph: &'glyph pixfont::Glyph,
    metrics: &'glyph pixfont::Metrics,
    scale: f32,
    offset: Vector<f32>,
    tool: Tool,
    guidelines: Vec<pixfont::Guidelines>,
    on_scale: Option<Box<dyn Fn(f32) -> Message + 'state>>,
    on_pan: Option<Box<dyn Fn(Vector<f32>) -> Message + 'state>>,
    colors: EditorColors,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    Pen,
    Line,
    Rectangle,
    Fill,
    Eraser,
    Pan,
}

#[derive(Debug, Default)]
struct State {
    will_pan: bool,
    pan_start: Option<Vector<f32>>,
    _is_pressed: bool,
    _delta: Delta,
}

#[derive(Debug, Default)]
pub struct Delta {
    pub add: HashSet<Point>,
    pub remove: HashSet<Point>,
}

impl<'state, 'glyph, Message> GlyphEditor<'state, 'glyph, Message> {
    pub fn new(glyph: &'glyph pixfont::Glyph, metrics: &'glyph pixfont::Metrics) -> Self {
        GlyphEditor {
            glyph,
            metrics,
            scale: 5.0,
            offset: Vector::new(0.0, 0.0),
            tool: Tool::Pen,
            on_scale: None,
            on_pan: None,
            guidelines: Vec::new(),
            colors: Default::default(),
        }
    }

    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    pub fn offset(mut self, offset: Vector<f32>) -> Self {
        self.offset = offset;
        self
    }

    pub fn guidelines(mut self, guidelines: pixfont::Guidelines) -> Self {
        self.guidelines.push(guidelines);
        self
    }

    pub fn colors(mut self, colors: EditorColors) -> Self {
        self.colors = colors;
        self
    }

    pub fn tool(mut self, tool: Tool) -> Self {
        self.tool = tool;
        self
    }

    pub fn on_scale(mut self, on_scale: impl Fn(f32) -> Message + 'state) -> Self {
        self.on_scale = Some(Box::new(on_scale));
        self
    }

    pub fn on_pan(mut self, on_pan: impl Fn(Vector<f32>) -> Message + 'state) -> Self {
        self.on_pan = Some(Box::new(on_pan));
        self
    }

    fn to_font_point(&self, point: Option<Point>, bounds: &Rectangle) -> Option<pixfont::Point> {
        if let Some(point) = point {
            let center = Vector::new(bounds.center_x(), bounds.center_y());
            let point =
                point - Vector::new(-self.scale / 2.0, -self.scale / 2.0) - self.offset - center;
            let point = Point::new(point.x / self.scale, point.y / self.scale);

            Some(pixfont::Point::new(
                point.x.floor() as i32,
                -(point.y.floor()) as i32,
            ))
        } else {
            None
        }
    }

    fn to_iced_rect(&self, point: Option<pixfont::Point>, bounds: &Rectangle) -> Option<Rectangle> {
        if let Some(point) = point {
            let center = bounds.center();
            let point = Vector::new(
                (point.x as f32) * self.scale,
                (-point.y as f32) * self.scale,
            );

            let point =
                center + self.offset + Vector::new(-self.scale / 2.0, -self.scale / 2.0) + point;

            Some(Rectangle::new(point, Size::new(self.scale, self.scale)))
        } else {
            None
        }
    }
}

pub const BACKGROUND_COLOR: Color = Color::from_rgb(0.2, 0.2, 0.2);
pub const GRIDLINE_COLOR: Color = Color::BLACK;
pub const GLYPH_COLOR: Color = Color::WHITE;
pub const ORIGIN_COLOR: Color = Color::from_rgb8(0x80, 0x20, 0x20);
pub const METRICS_COLOR: Color = Color::from_rgb8(0x80, 0xFF, 0);
pub const GUIDELINE_COLOR: Color = Color::from_rgb8(0x80, 0x00, 0xFF);

impl<Message, Theme, Renderer: renderer::Renderer> Widget<Message, Theme, Renderer>
    for GlyphEditor<'_, '_, Message>
{
    fn size(&self) -> iced::Size<iced::Length> {
        Size {
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<State>()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(State::default())
    }

    fn layout(
        &mut self,
        _tree: &mut iced::advanced::widget::Tree,
        _renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        layout::Node::new(limits.max())
    }

    fn draw(
        &self,
        _tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        _cursor: iced::advanced::mouse::Cursor,
        _viewport: &iced::Rectangle,
    ) {
        let bounds = layout.bounds();
        Draw::with(bounds, renderer, |draw| {
            draw.renderer.fill_quad(
                Quad {
                    bounds,
                    ..Default::default()
                },
                self.colors.background.map_or(BACKGROUND_COLOR, Into::into),
            );

            // pixels
            for pixel in self.glyph.pixels.pixels() {
                if let Some(origin) = self.to_iced_rect(Some(*pixel), &bounds) {
                    draw.renderer.fill_quad(
                        Quad {
                            bounds: origin,
                            ..Default::default()
                        },
                        self.colors.glyph.map_or(GLYPH_COLOR, Into::into),
                    );
                }
            }

            // gridlines
            draw.color = self
                .colors
                .gridlines
                .map_or(GRIDLINE_COLOR, Into::into)
                .into();
            let mut gx = bounds.x + self.offset.x % self.scale + (bounds.width / 2.0) % self.scale
                - self.scale * 0.5;
            let mut gy = bounds.y
                + self.offset.y % self.scale
                + (bounds.height / 2.0) % (self.scale)
                + self.scale * 0.5;
            while gx <= bounds.x + bounds.width {
                draw.vline(gx);
                gx += self.scale;
            }
            while gy <= bounds.y + bounds.height {
                draw.hline(gy);
                gy += self.scale;
            }

            // guidelines
            for guidelines in &self.guidelines {
                draw.color =
                    Background::Color(self.colors.guidelines.map_or(GUIDELINE_COLOR, Into::into));

                for pixfont::Guideline { position, .. } in &guidelines.x {
                    draw.vline(
                        bounds.center_x() + self.offset.x - (0.5 * self.scale)
                            + ((*position as f32) * self.scale),
                    );
                }

                for pixfont::Guideline { position, .. } in &guidelines.y {
                    draw.hline(
                        bounds.center_y() + self.offset.y + (0.5 * self.scale)
                            - ((*position as f32) * self.scale),
                    );
                }
            }

            // metrics lines
            draw.color = self.colors.metrics.map_or(METRICS_COLOR, Into::into).into();
            draw.vline(
                bounds.center_x() + self.offset.x - self.scale / 2.0
                    + (self.glyph.advance as f32) * self.scale,
            );
            draw.hline(
                bounds.center_y() + self.offset.y + self.scale / 2.0
                    - (self.metrics.ascender as f32) * self.scale,
            );
            draw.hline(
                bounds.center_y() + self.offset.y + self.scale / 2.0
                    - (self.metrics.descender as f32) * self.scale,
            );
            draw.hline(
                bounds.center_y() + self.offset.y + self.scale / 2.0
                    - (self.metrics.cap_height as f32) * self.scale,
            );
            draw.hline(
                bounds.center_y() + self.offset.y + self.scale / 2.0
                    - (self.metrics.x_height as f32) * self.scale,
            );

            // origin lines
            draw.color = self.colors.origin.map_or(ORIGIN_COLOR, Into::into).into();
            draw.vline(bounds.center_x() + self.offset.x - self.scale / 2.0);
            draw.hline(bounds.center_y() + self.offset.y + self.scale / 2.0);
        });
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

        let state = _tree.state.downcast_ref::<State>();

        if state.will_pan {
            return match state.pan_start {
                Some(_) => iced::advanced::mouse::Interaction::Grabbing,
                None => iced::advanced::mouse::Interaction::Grab,
            };
        }

        iced::advanced::mouse::Interaction::Crosshair
    }

    fn update(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        event: &iced::Event,
        layout: layout::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        //println!("{:?}", event);
        match event {
            iced::Event::Keyboard(event) => match event {
                iced::keyboard::Event::KeyPressed {
                    key,
                    modified_key: _modified_key,
                    physical_key: _physical_key,
                    location: _location,
                    modifiers: _modifiers,
                    text: _text,
                    repeat: _repeat,
                } => {
                    let state = tree.state.downcast_mut::<State>();

                    if *key == Key::Named(key::Named::Space) {
                        state.will_pan = true;
                    }
                }
                iced::keyboard::Event::KeyReleased {
                    key,
                    modified_key: _modified_key,
                    physical_key: _physical_key,
                    location: _location,
                    modifiers: _modifiers,
                } => {
                    let state = tree.state.downcast_mut::<State>();

                    if *key == Key::Named(key::Named::Space) {
                        state.will_pan = false;
                        state.pan_start = None;
                    }
                }
                iced::keyboard::Event::ModifiersChanged(_modifiers) => {}
            },
            iced::Event::Mouse(event) => match event {
                iced::mouse::Event::CursorEntered => {}
                iced::mouse::Event::CursorLeft => {}
                iced::mouse::Event::CursorMoved { position } => {
                    let state = tree.state.downcast_mut::<State>();

                    if let Some(last_pan) = state.pan_start
                        && let Some(on_pan) = &self.on_pan
                    {
                        let delta = *position - last_pan;
                        let delta = Vector::new(delta.x, delta.y);
                        shell.publish(on_pan(self.offset + delta));

                        state.pan_start = Some(Vector::new(position.x, position.y))
                    }
                }
                iced::mouse::Event::ButtonPressed(_button) => {
                    let state = tree.state.downcast_mut::<State>();

                    let bounds = layout.bounds();

                    if cursor.is_over(bounds) {
                        if state.will_pan
                            && let Some(position) = cursor.position()
                        {
                            state.pan_start = Some(Vector::new(position.x, position.y));
                            return;
                        }

                        if let Some(point) = self.to_font_point(cursor.position(), &bounds) {
                            print!("clicked {:?}", point);
                        }
                    }

                    // TODO: dispatch based on what needs to be done
                }
                iced::mouse::Event::ButtonReleased(_button) => {
                    let state = tree.state.downcast_mut::<State>();

                    if let Some(last_pan) = state.pan_start
                        && let Some(position) = cursor.position()
                        && let Some(on_pan) = &self.on_pan
                    {
                        let delta = position - last_pan;
                        let delta = Vector::new(delta.x, delta.y);
                        shell.publish(on_pan(self.offset + delta));
                    }
                    state.pan_start = None;
                }
                iced::mouse::Event::WheelScrolled { delta } => {
                    let y = match delta {
                        iced::mouse::ScrollDelta::Lines { x: _x, y } => *y,
                        iced::mouse::ScrollDelta::Pixels { x: _x, y } => *y,
                    };

                    let message = if let Some(on_scale) = &self.on_scale {
                        match y.total_cmp(&0.0) {
                            Ordering::Less => Some(on_scale(f32::max(2.0, self.scale * 0.9))),
                            Ordering::Greater => Some(on_scale(self.scale * 1.1)),
                            _ => None,
                        }
                    } else {
                        None
                    };

                    if let Some(message) = message {
                        if cursor.is_over(layout.bounds()) {
                            shell.publish(message);
                        }
                        shell.capture_event();
                    }
                }
            },
            iced::Event::Window(_event) => {}
            iced::Event::Touch(_event) => {}
            iced::Event::InputMethod(_event) => {}
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

struct Draw<'draw, Renderer: renderer::Renderer> {
    renderer: &'draw mut Renderer,
    bounds: Rectangle,
    color: Background,
}

impl<'draw, Renderer: renderer::Renderer> Draw<'draw, Renderer> {
    fn with(
        bounds: Rectangle,
        renderer: &mut Renderer,
        receiver: impl for<'inner> FnOnce(&mut Draw<'inner, Renderer>),
    ) {
        renderer.with_layer(bounds, |renderer: &mut Renderer| {
            let mut draw = Draw {
                renderer,
                bounds,
                color: Color::BLACK.into(),
            };
            receiver(&mut draw)
        });
    }

    fn hline(&mut self, y: f32) {
        self.renderer.fill_quad(
            Quad {
                bounds: Rectangle {
                    x: self.bounds.x,
                    y,
                    width: self.bounds.width,
                    height: 1.0,
                },
                ..Default::default()
            },
            self.color,
        );
    }

    fn vline(&mut self, x: f32) {
        self.renderer.fill_quad(
            Quad {
                bounds: Rectangle {
                    x,
                    y: self.bounds.y,
                    width: 1.0,
                    height: self.bounds.height,
                },
                ..Default::default()
            },
            self.color,
        );
    }
}
