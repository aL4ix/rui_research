use std::error::Error;
use std::fmt::Debug;

use glyph_brush::ab_glyph::FontArc;

use crate::general::{Color, Geometry, Vector2D};
use crate::widgets::{Primitive, Text, Widget};
use crate::widgets::events::MouseButtonDown;
use crate::widgets::primitive::private::PrivatePrimitiveMethods;
use crate::window::WindowBuilder;

#[derive(Debug)]
pub struct TextBox {
    id: usize,
    position: Vector2D<f32>,
    size: Vector2D<f32>,
    text: Text,
    geometry: Geometry,
    needs_update: bool,
    needs_translation: bool,
    translated_geometry: Geometry,
    event_mouse_button_down: MouseButtonDown,
}

impl TextBox {
    pub fn new(id: usize, text: &str, font_size: f32, font: FontArc, color: Color) -> TextBox {
        let text = Text::new(0, text, font_size, font, color);
        let size = text.size().clone();
        TextBox {
            id,
            position: Default::default(),
            size,
            text,
            geometry: Default::default(),
            needs_update: true, // TODO don't be lazy, also is there a way to force not being lazy?
            needs_translation: true,
            translated_geometry: Default::default(),
            event_mouse_button_down: Default::default(),
        }
    }
    pub fn get_by_id(window: &mut WindowBuilder, id: usize) -> Result<&mut TextBox, Box<dyn Error>> {
        if let Some(widget) = window.get_widget_by_id(id) {
            return if let Some(text_box) = widget.downcast_mut::<TextBox>() {
                Ok(text_box)
            } else {
                Err(Box::from("Not a Text"))
            };
        }
        Err(Box::from("Not found"))
    }
    pub fn set_text(&mut self, text: &str) {
        self.text.set_text(text);
        self.needs_update = true;
    }
}

impl Primitive for TextBox {
    fn id(&self) -> usize {
        self.id
    }
    fn set_id(&mut self, id: usize) {
        self.id = id
    }
    fn x(&self) -> f32 {
        self.position.x()
    }
    fn y(&self) -> f32 {
        self.position.y()
    }
    fn position(&self) -> &Vector2D<f32> {
        &self.position
    }
    fn set_position(&mut self, position: Vector2D<f32>) {
        self.position = position
    }
    fn width(&self) -> f32 {
        self.size.x()
    }
    fn height(&self) -> f32 {
        self.size.y()
    }
    fn size(&self) -> &Vector2D<f32> {
        &self.size
    }
}

impl PrivatePrimitiveMethods for TextBox {
    fn update_geometry(&mut self) {
        self.geometry = self.text.build_geometry()
    }
    fn needs_update(&self) -> bool {
        self.needs_update
    }
    fn set_needs_update(&mut self, needs_update: bool) {
        self.needs_update = needs_update
    }
    fn needs_translation(&self) -> bool {
        self.needs_translation
    }
    fn set_needs_translation(&mut self, needs_translation: bool) {
        self.needs_translation = needs_translation
    }
    fn clone_geometry(&self) -> Geometry {
        self.geometry.clone()
    }
    fn set_translated_geometry(&mut self, translated_geometry: Geometry) {
        self.translated_geometry = translated_geometry
    }
    fn clone_translated_geometry(&self) -> Geometry {
        self.translated_geometry.clone()
    }
}

impl Widget for TextBox {
    fn event_mouse_button_down(&mut self, x: i32, y: i32) {
        (self.event_mouse_button_down.callback)(self, x, y)
    }
    fn set_event_mouse_button_down(&mut self, callback: fn(&mut dyn Widget, i32, i32)) {
        self.event_mouse_button_down = MouseButtonDown {
            callback
        };
    }
}