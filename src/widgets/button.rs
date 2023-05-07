use std::error::Error;
use std::fmt::Debug;

use crate::general::{Geometry, Vector2D};
use crate::widgets::{CommonWidget, Widget};
use crate::widgets::primitives::Primitive;
use crate::widgets::primitives::private::PrivatePrimitiveMethods;
use crate::widgets::themes::StyleMaster;
use crate::window::WindowBuilder;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Button {
    common: CommonWidget,
    text_index: usize,
}

impl Button {
    pub fn new(id: usize, text: &str, style: &StyleMaster) -> Result<Button, Box<dyn Error>> {
        let (size, primitives, text_index) = style.one_button(Vector2D::default(), text)?;
        Ok(Button {
            common: CommonWidget::new(id, Self::class_name(), primitives, size),
            text_index,
        })
    }
    #[allow(dead_code)]
    pub fn get_by_id(window: &mut WindowBuilder, id: usize) -> Result<&mut Button, Box<dyn Error>> {
        if let Some(widget) = window.get_widget_by_id(id) {
            return if let Some(button) = widget.downcast_mut::<Button>() {
                Ok(button)
            } else {
                Err(Box::from(format!("Not a {}", Self::class_name())))
            };
        }
        Err(Box::from("Not found"))
    }
}


impl Primitive for Button {
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

impl PrivatePrimitiveMethods for Button {
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

impl Widget for Button {
    fn class_name() -> &'static str {
        "Button"
    }
    fn event_mouse_button_down(&mut self, x: i32, y: i32) {
        self.common.event_mouse_button_down(x, y)
    }
    fn set_event_mouse_button_down(&mut self, callback: fn(this: &mut dyn Widget, x: i32, y: i32)) {
        self.common.set_event_mouse_button_down(callback)
    }
}