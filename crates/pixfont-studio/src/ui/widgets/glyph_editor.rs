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
    mouse::Interaction,
};

use crate::settings::EditorColors;

pub struct GlyphEditor<'state, 'glyph, Message> {
    glyph: &'glyph pixfont::Glyph,
    metrics: &'glyph pixfont::Metrics,
    scale: f32,
    offset: Vector<f32>,
    tool: Tool,
    guidelines: Vec<pixfont::Guidelines>,
    on_scale: Option<Box<dyn Fn(f32) -> Message + 'state>>,
    on_pan: Option<Box<dyn Fn(Vector<f32>) -> Message + 'state>>,
    on_tool: Option<Box<dyn Fn(Tool) -> Message + 'state>>,
    on_apply: Option<Box<dyn Fn(Delta) -> Message + 'state>>,
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
    pointer: ToolState,
    pan_replaced: Option<Tool>,
}

#[derive(Debug, Default)]
pub enum ToolState {
    #[default]
    None,
    Pen {
        delta: Delta,
    },
    Line {
        start: pixfont::Point,
        end: pixfont::Point,
    },
    Rectangle {
        start: pixfont::Point,
        end: pixfont::Point,
        fill: bool,
    },
    Fill {
        point: pixfont::Point,
    },
    Eraser {
        delta: Delta,
    },
    Pan {
        pan_start: Vector<f32>,
        delta: Vector<f32>,
    },
}

impl ToolState {
    pub fn delta(&self) -> Delta {
        match self {
            ToolState::None => Default::default(),
            ToolState::Pen { delta } => delta.clone(),
            ToolState::Line { start, end } => {
                let mut delta = Delta::default();
                bresenham(*start, *end, |point| {
                    delta.add.insert(point);
                });
                delta
            }
            ToolState::Rectangle { start, end, fill } => {
                let mut delta = Delta::default();

                let min_x = start.x.min(end.x);
                let min_y = start.y.min(end.y);
                let max_x = start.x.max(end.x);
                let max_y = start.y.max(end.y);

                if *fill {
                    for x in min_x..=max_x {
                        for y in min_y..=max_y {
                            delta.add.insert(pixfont::Point::new(x, y));
                        }
                    }
                } else {
                    for x in min_x..=max_x {
                        delta.add.insert(pixfont::Point { x, y: start.y });
                        delta.add.insert(pixfont::Point { x, y: end.y });
                    }
                    for y in min_y..=max_y {
                        delta.add.insert(pixfont::Point { x: start.x, y });
                        delta.add.insert(pixfont::Point { x: end.x, y });
                    }
                }

                delta
            }
            ToolState::Fill { point } => {
                _ = point;
                todo!()
            }
            ToolState::Eraser { delta } => delta.clone(),
            ToolState::Pan { .. } => Default::default(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Delta {
    pub add: HashSet<pixfont::Point>,
    pub remove: HashSet<pixfont::Point>,
}

impl<'state, 'glyph, Message> GlyphEditor<'state, 'glyph, Message> {
    pub fn new(glyph: &'glyph pixfont::Glyph, metrics: &'glyph pixfont::Metrics) -> Self {
        GlyphEditor {
            glyph,
            metrics,
            scale: 5.0,
            offset: Vector::new(0.0, 0.0),
            tool: Tool::Pen,
            on_scale: Default::default(),
            on_pan: Default::default(),
            on_tool: Default::default(),
            on_apply: Default::default(),
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

    pub fn on_tool(mut self, on_tool: impl Fn(Tool) -> Message + 'state) -> Self {
        self.on_tool = Some(Box::new(on_tool));
        self
    }

    pub fn on_apply(mut self, on_apply: impl Fn(Delta) -> Message + 'state) -> Self {
        self.on_apply = Some(Box::new(on_apply));
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
        tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        _cursor: iced::advanced::mouse::Cursor,
        _viewport: &iced::Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();

        let bounds = layout.bounds();
        let offset = self.offset
            + match &state.pointer {
                ToolState::Pan { delta, .. } => *delta,
                _ => Vector::ZERO,
            };

        Draw::with(bounds, renderer, |draw| {
            draw.renderer.fill_quad(
                Quad {
                    bounds,
                    ..Default::default()
                },
                self.colors.background.map_or(BACKGROUND_COLOR, Into::into),
            );

            // pixels
            let delta = state.pointer.delta();
            for pixel in self
                .glyph
                .pixels
                .pixels()
                .chain(delta.add.iter())
                .filter(|item| !delta.remove.contains(*item))
            {
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
            let mut gx = bounds.x + offset.x % self.scale + (bounds.width / 2.0) % self.scale
                - self.scale * 0.5;
            let mut gy = bounds.y
                + offset.y % self.scale
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
                        bounds.center_x() + offset.x - (0.5 * self.scale)
                            + ((*position as f32) * self.scale),
                    );
                }

                for pixfont::Guideline { position, .. } in &guidelines.y {
                    draw.hline(
                        bounds.center_y() + offset.y + (0.5 * self.scale)
                            - ((*position as f32) * self.scale),
                    );
                }
            }

            // metrics lines
            draw.color = self.colors.metrics.map_or(METRICS_COLOR, Into::into).into();
            draw.vline(
                bounds.center_x() + offset.x - self.scale / 2.0
                    + (self.glyph.advance as f32) * self.scale,
            );
            draw.hline(
                bounds.center_y() + offset.y + self.scale / 2.0
                    - (self.metrics.ascender as f32) * self.scale,
            );
            draw.hline(
                bounds.center_y() + offset.y + self.scale / 2.0
                    - (self.metrics.descender as f32) * self.scale,
            );
            draw.hline(
                bounds.center_y() + offset.y + self.scale / 2.0
                    - (self.metrics.cap_height as f32) * self.scale,
            );
            draw.hline(
                bounds.center_y() + offset.y + self.scale / 2.0
                    - (self.metrics.x_height as f32) * self.scale,
            );

            // origin lines
            draw.color = self.colors.origin.map_or(ORIGIN_COLOR, Into::into).into();
            draw.vline(bounds.center_x() + offset.x - self.scale / 2.0);
            draw.hline(bounds.center_y() + offset.y + self.scale / 2.0);
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

        match self.tool {
            Tool::Pen => Interaction::Crosshair,
            Tool::Line => Interaction::Crosshair,
            Tool::Rectangle => Interaction::Crosshair,
            Tool::Fill => Interaction::Crosshair,
            Tool::Eraser => Interaction::Crosshair,
            Tool::Pan => {
                if let ToolState::Pan { .. } = state.pointer {
                    Interaction::Grabbing
                } else {
                    Interaction::Grab
                }
            }
        }
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
        match event {
            iced::Event::Keyboard(event) => match event {
                iced::keyboard::Event::KeyPressed {
                    key,
                    modified_key: _modified_key,
                    physical_key: _physical_key,
                    location: _location,
                    modifiers: _modifiers,
                    text: _text,
                    repeat,
                } => {
                    let state = tree.state.downcast_mut::<State>();

                    if *key == Key::Named(key::Named::Space) {
                        if *repeat {
                            return;
                        }

                        state.pan_replaced = Some(self.tool);
                        if let Some(on_tool) = &self.on_tool {
                            shell.publish(on_tool(Tool::Pan));
                        }

                        shell.capture_event();
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
                        if let Some(on_tool) = &self.on_tool {
                            shell.publish(on_tool(state.pan_replaced.unwrap_or(Tool::Pan)));
                        }
                        state.pan_replaced = None;
                        shell.capture_event();
                    }
                }
                iced::keyboard::Event::ModifiersChanged(_modifiers) => {}
            },

            iced::Event::Mouse(event) => match event {
                iced::mouse::Event::CursorEntered => {}
                iced::mouse::Event::CursorLeft => {}
                iced::mouse::Event::CursorMoved { position } => {
                    let bounds = layout.bounds();
                    let state = tree.state.downcast_mut::<State>();
                    let font_point = self.to_font_point(Some(*position), &bounds);

                    match &mut state.pointer {
                        ToolState::None => {}

                        ToolState::Pen { delta } => {
                            let Some(font_point) = font_point else {
                                return;
                            };

                            delta.add.insert(font_point);
                            shell.request_redraw();
                        }

                        ToolState::Line { end, .. } => {
                            if let Some(font_point) = font_point {
                                *end = font_point;
                                shell.request_redraw();
                            };
                        }

                        ToolState::Rectangle { end, .. } => {
                            if let Some(font_point) = font_point {
                                *end = font_point;
                            }
                            shell.request_redraw();
                        }

                        ToolState::Fill { .. } => todo!(),

                        ToolState::Eraser { delta } => {
                            if let Some(font_point) = font_point {
                                delta.remove.insert(font_point);
                                shell.request_redraw();
                            }
                        }

                        ToolState::Pan { pan_start, delta } => {
                            let d = *position - *pan_start;
                            *delta = Vector::new(d.x, d.y);
                            shell.request_redraw();
                        }
                    }
                }

                iced::mouse::Event::ButtonPressed(_button) => {
                    let bounds = layout.bounds();
                    if !cursor.is_over(bounds) {
                        return;
                    }

                    let cursor_position = cursor.position().unwrap();
                    let font_point = self.to_font_point(Some(cursor_position), &bounds);

                    let state = tree.state.downcast_mut::<State>();

                    state.pointer = match self.tool {
                        Tool::Pen => ToolState::Pen {
                            delta: Default::default(),
                        },
                        Tool::Line => ToolState::Line {
                            start: font_point.unwrap(),
                            end: font_point.unwrap(),
                        },
                        Tool::Rectangle => ToolState::Rectangle {
                            start: font_point.unwrap(),
                            end: font_point.unwrap(),
                            fill: false,
                        },
                        Tool::Fill => ToolState::Fill {
                            point: font_point.unwrap(),
                        },
                        Tool::Eraser => ToolState::Eraser {
                            delta: Default::default(),
                        },
                        Tool::Pan => ToolState::Pan {
                            pan_start: Vector::new(cursor_position.x, cursor_position.y),
                            delta: Vector::ZERO,
                        },
                    };

                    shell.capture_event();

                    // TODO: dispatch based on what needs to be done
                }

                iced::mouse::Event::ButtonReleased(_button) => {
                    let state = tree.state.downcast_mut::<State>();

                    match &state.pointer {
                        ToolState::None => return,

                        ToolState::Pan { delta, .. } => {
                            let on_pan = self.on_pan.as_ref().unwrap();
                            shell.publish(on_pan(self.offset + *delta));
                        }

                        _ => {
                            if let Some(on_apply) = &self.on_apply {
                                shell.publish(on_apply(state.pointer.delta()));
                            }
                        }
                    };

                    state.pointer = ToolState::None;
                    shell.capture_event();
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

fn bresenham(start: pixfont::Point, end: pixfont::Point, mut put: impl FnMut(pixfont::Point)) {
    let dx = end.x - start.x;
    let ax = 2 * if dx < 0 { -dx } else { dx };
    let sx = if dx < 0 { -1 } else { 1 };

    let dy = end.y - start.y;
    let ay = 2 * if dy < 0 { -dy } else { dy };
    let sy = if dy < 0 { -1 } else { 1 };

    let mut x = start.x;
    let mut y = start.y;

    if ax > ay {
        let mut d = ay - ax / 2;
        loop {
            put(pixfont::Point::new(x, y));
            if x == end.x {
                return;
            }

            if d >= 0 {
                y += sy;
                d -= ax;
            }

            x += sx;
            d += ay;
        }
    } else {
        let mut d = ax - ay / 2;
        loop {
            put(pixfont::Point::new(x, y));
            if y == end.y {
                return;
            }

            if d >= 0 {
                x += sx;
                d -= ay;
            }

            y += sy;
            d += ax;
        }
    }
}
