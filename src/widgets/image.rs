use std::any::TypeId;
use std::error::Error;
use std::fmt::Debug;
use std::path::Path;
use std::sync::Arc;

use crate::general::{Geometry, Vector2D};
use crate::themes::{PrimitivesManagerForThemes, StyleMaster, ThemeForImage, ThemeStyleForImage};
use crate::widgets::events::MouseButtonDownCallback;
use crate::widgets::primitives::private::PrivatePrimitiveMethods;
use crate::widgets::primitives::Primitive;
use crate::widgets::{CommonWidget, WidgetEnum};

use super::events::HasEvents;
use super::Widget;

#[derive(Debug)]
pub struct Image {
    common: CommonWidget,
}

impl Image {
    pub fn from_bmp<WENUM: WidgetEnum>(
        wid: WENUM,
        path: Box<Path>,
        style_master: Arc<StyleMaster>,
    ) -> Result<Image, Box<dyn Error>> {
        let theme: &dyn ThemeForImage =
            style_master.expect_theme_for_widget_t(TypeId::of::<Self>());
        let style: Box<ThemeStyleForImage> =
            style_master.expect_style_for_widget_t(Self::class_name());
        let mut prim_man = PrimitivesManagerForThemes::new();
        let size = theme.new_image(path, None, style, &mut prim_man);
        Ok(Image {
            common: CommonWidget::new(wid, Self::class_name(), size, style_master, prim_man),
        })
    }
}

impl Primitive for Image {
    fn class_name() -> &'static str {
        "Image"
    }
    // TODO change to macro
    fn class(&self) -> &'static str {
        Self::class_name()
    }
    fn wid(&self) -> usize {
        self.common.wid()
    }
    fn set_wid(&mut self, nid: usize) {
        self.common.set_wid(nid)
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

impl PrivatePrimitiveMethods for Image {
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

impl HasEvents for Image {
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

impl Widget for Image {}
