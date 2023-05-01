use std::error::Error;
use std::fmt::Debug;

use glyph_brush::ab_glyph::FontArc;

use crate::general::{Color, Geometry, Vector2D};
use crate::widgets::{CommonWidget, Primitive, Widget};
use crate::widgets::primitives::private::PrivatePrimitiveMethods;
use crate::widgets::primitives::Text;
use crate::window::WindowBuilder;

#[derive(Debug)]
pub struct TextBox {
    common: CommonWidget,
}

impl TextBox {
    pub fn new(id: usize, text: &str, font_size: f32, font: FontArc, color: Color) -> TextBox {
        let text = Text::new(0, text, font_size, font, color);
        let size = text.size().clone();
        let primitives: Vec<Box<dyn Primitive>> = vec![Box::new(text)];
        let common_widget = CommonWidget::new(id, "TextBox", primitives, size);
        TextBox {
            common: common_widget
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
        let primitive = self.common.get_primitive_by_id(0);
        let p_text: &mut Text = primitive.downcast_mut::<Text>().expect("downcast");
        // TODO how to improve this?
        p_text.set_text(text);
        self.set_needs_update(true);
    }
}

impl Primitive for TextBox {
    fn id(&self) -> usize {
        self.common.id()
    }
    fn set_id(&mut self, id: usize) {
        self.common.set_id(id)
    }
    fn x(&self) -> f32 {
        self.common.x()
    }
    fn y(&self) -> f32 {
        self.common.y()
    }
    fn position(&self) -> &Vector2D<f32> {
        self.common.position()
    }
    fn set_position(&mut self, position: Vector2D<f32>) {
        self.common.set_position(position)
    }
    fn width(&self) -> f32 {
        self.common.width()
    }
    fn height(&self) -> f32 {
        self.common.height()
    }
    fn size(&self) -> &Vector2D<f32> {
        self.common.size()
    }
}

impl PrivatePrimitiveMethods for TextBox {
    fn update_geometry(&mut self) {
        self.common.update_geometry()
    }
    fn needs_update(&self) -> bool {
        self.common.needs_update()
    }
    fn set_needs_update(&mut self, needs_update: bool) {
        self.common.set_needs_update(needs_update)
    }
    fn needs_translation(&self) -> bool {
        self.common.needs_translation()
    }
    fn set_needs_translation(&mut self, needs_translation: bool) {
        self.common.set_needs_translation(needs_translation)
    }
    fn clone_geometry(&self) -> Geometry {
        self.common.clone_geometry()
    }
    fn set_translated_geometry(&mut self, translated_geometry: Geometry) {
        self.common.set_translated_geometry(translated_geometry)
    }
    fn clone_translated_geometry(&self) -> Geometry {
        self.common.clone_translated_geometry()
    }
}

impl Widget for TextBox {
    fn event_mouse_button_down(&mut self, x: i32, y: i32) {
        self.common.event_mouse_button_down(x, y)
    }
    fn set_event_mouse_button_down(&mut self, callback: fn(&mut dyn Widget, i32, i32)) {
        self.common.set_event_mouse_button_down(callback)
    }
}