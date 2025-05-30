use std::any::TypeId;
use std::error::Error;
use std::fmt::Debug;
use std::sync::Arc;

use crate::general::{Geometry, Vector2D};
use crate::widgets::events::MouseButtonDownCallback;
use crate::widgets::primitives::private::PrivatePrimitiveMethods;
use crate::widgets::primitives::Primitive;
use crate::widgets::themes::StyleMaster;
use crate::widgets::{CommonWidget, StyleForButton, ThemeForButton};

use super::events::HasEvents;
use super::{PrimitiveManagerForThemes, Widget};

#[derive(Debug)]
pub struct Button {
    common: CommonWidget,
}

impl Button {
    pub fn new(
        nid: usize,
        text: &str,
        style_master: Arc<StyleMaster>,
    ) -> Result<Button, Box<dyn Error>> {
        let theme: &dyn ThemeForButton =
            style_master.expect_theme_for_widget_t(TypeId::of::<Self>());
        let style: Box<StyleForButton> = style_master.expect_style_for_widget_t(Self::class_name());
        let mut prim_man = PrimitiveManagerForThemes::new();
        let size = theme.new(text, None, style, &mut prim_man);
        Ok(Button {
            common: CommonWidget::new(nid, Self::class_name(), size, style_master, prim_man),
        })
    }
    pub fn set_text(&mut self, text: &str) {
        let binding = self.common.style_master();
        let theme: &dyn ThemeForButton = binding.expect_theme_for_widget_t(TypeId::of::<Self>());
        let style: Box<StyleForButton> = binding.expect_style_for_widget_t(Self::class_name());
        let size = theme.set_text(text, None, style, &mut self.common.prim_man());
        self.common.set_size(size);
        self.set_needs_update(true);
    }
}

impl Primitive for Button {
    fn class_name() -> &'static str {
        "Button"
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
    fn size(&mut self) -> &Vector2D<f32> {
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

impl HasEvents for Button {
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

impl Widget for Button {}
