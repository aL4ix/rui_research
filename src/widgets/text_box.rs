use std::error::Error;
use std::fmt::Debug;
use std::sync::Arc;

use crate::general::{Geometry, Vector2D};
use crate::utils::Downcast;
use crate::widgets::events::MouseButtonDownCallback;
use crate::widgets::primitives::private::PrivatePrimitiveMethods;
use crate::widgets::primitives::Text;
use crate::widgets::themes::StyleMaster;
use crate::widgets::{CommonWidget, Primitive};

use super::events::HasEvents;
use super::Widget;

#[derive(Debug)]
pub struct TextBox {
    common: CommonWidget,
    text_index: usize,
}

impl TextBox {
    pub fn new(nid: usize, text: &str, style: &StyleMaster) -> Result<TextBox, Box<dyn Error>> {
        let (size, primitives, text_index) = style.one_textbox(Vector2D::default(), text)?;
        let common_widget = CommonWidget::new(nid, Self::class_name(), primitives, size);
        Ok(TextBox {
            common: common_widget,
            text_index,
        })
    }
    pub fn set_text(&mut self, text: &str) {
        let primitive = self.common.get_primitive_by_index_mut(self.text_index);
        let text_pri: &mut Text = (**primitive)
            .downcast_mut::<Text>()
            .expect("text_box.set_text() downcast_mut");
        text_pri.set_text(text);
        self.set_needs_update(true);
    }
}

impl Primitive for TextBox {
    fn class_name() -> &'static str {
        "TextBox"
    }
    // TODO change to macro
    fn class(&self) -> &'static str {
        Self::class_name()
    }
    fn nid(&self) -> usize {
        self.common.nid()
    }
    fn set_nid(&mut self, nid: usize) {
        self.common.set_nid(nid)
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

impl HasEvents for TextBox {
    fn event_mouse_button_down(&self) -> Arc<MouseButtonDownCallback> {
        self.common.event_mouse_button_down()
    }
    fn set_event_mouse_button_down(&mut self, callback: MouseButtonDownCallback) {
        self.common.set_event_mouse_button_down(callback)
    }
    fn event_key_down(&self) -> Arc<super::events::KeyDownCallback> {
        self.common.event_key_down()
    }
    fn set_event_key_down(&mut self, callback: super::events::KeyDownCallback) {
        self.common.set_event_key_down(callback);
    }
}

impl Widget for TextBox {}
