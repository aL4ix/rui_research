use std::error::Error;
use std::fmt::Debug;

use glyph_brush::ab_glyph::FontArc;

use crate::general::{Color, Geometry, Vector2D};
use crate::widgets::{Shape, Text, Primitive};
use crate::widgets::Widget;
use crate::widgets::events::MouseButtonDown;
use crate::widgets::primitive::private::PrivatePrimitiveMethods;
use crate::window::WindowBuilder;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Button {
    id: usize,
    position: Vector2D<f32>,
    size: Vector2D<f32>,
    shape: Shape,
    text: Text,
    geometry: Geometry,
    needs_update: bool,
    needs_translation: bool,
    translated_geometry: Geometry,
    event_mouse_button_down: MouseButtonDown,
}

impl Button {
    pub fn new(id: usize, text: &str, font_size: f32, font: FontArc, color_t: Color, color_s: Color) -> Button {
        // TODO add theming
        let text = Text::new(0, text, font_size, font, color_t);
        let size = text.size();
        Button {
            id,
            position: Default::default(),
            size: size.clone(),
            shape: Shape::new_square(0, size.clone(), 0, color_s),
            text,
            geometry: Default::default(),
            needs_update: true, // TODO don't be lazy, also is there a way to force not being lazy?
            needs_translation: true,
            translated_geometry: Default::default(),
            event_mouse_button_down: Default::default(),
        }
    }
    #[allow(dead_code)]
    pub fn get_by_id(window: &mut WindowBuilder, id: usize) -> Result<&mut Button, Box<dyn Error>> {
        if let Some(widget) = window.get_widget_by_id(id) {
            return if let Some(button) = widget.downcast_mut::<Button>() {
                Ok(button)
            } else {
                Err(Box::from("Not a Button"))
            };
        }
        Err(Box::from("Not found"))
    }
}


impl Primitive for Button {
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

impl PrivatePrimitiveMethods for Button {
    fn update_geometry(&mut self) {
        let shape_geo = self.shape.build_geometry();
        let text_geo = self.text.build_geometry();
        let geometries = vec![shape_geo, text_geo];
        self.geometry = Geometry::new_from_geometries("Button", geometries);
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

impl Widget for Button {
    fn event_mouse_button_down(&mut self, x: i32, y: i32) {
        (self.event_mouse_button_down.callback)(self, x, y)
    }
    fn set_event_mouse_button_down(&mut self, callback: fn(this: &mut dyn Widget, x: i32, y: i32)) {
        self.event_mouse_button_down = MouseButtonDown {
            callback,
        }
    }
}