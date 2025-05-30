use std::sync::Arc;

use crate::general::{Geometry, Vector2D};
use crate::widgets::events::{Event, MouseButtonDown, MouseButtonDownCallback};
use crate::widgets::primitives::private::PrivatePrimitiveMethods;
use crate::widgets::Primitive;

use super::events::{HasEvents, KeyDown, KeyDownCallback};
use super::themes::StyleMaster;
use super::{PrimitiveManagerForThemes, Widget};

#[derive(Debug)]
pub struct CommonWidget {
    nid: usize,
    position: Vector2D<f32>,
    size: Vector2D<f32>,
    geometry: Geometry,
    needs_update: bool,
    needs_translation: bool,
    translated_geometry: Geometry,
    event_mouse_button_down: MouseButtonDown,
    event_key_down: KeyDown,
    class: String,
    style_master: Arc<StyleMaster>,
    prim_man: PrimitiveManagerForThemes,
}

impl CommonWidget {
    pub fn new(
        nid: usize,
        class: &str,
        size: Vector2D<f32>,
        style_master: Arc<StyleMaster>,
        mut prim_man: PrimitiveManagerForThemes,
    ) -> CommonWidget {
        let geometry = Geometry::new_from_prim_man(class, &mut prim_man);
        CommonWidget {
            nid,
            position: Default::default(),
            size,
            geometry,
            needs_update: false,
            needs_translation: true,
            translated_geometry: Default::default(),
            event_mouse_button_down: Default::default(),
            event_key_down: Default::default(),
            class: class.to_string(),
            style_master,
            prim_man,
        }
    }
    pub fn style_master(&self) -> Arc<StyleMaster> {
        self.style_master.clone()
    }
    pub fn prim_man(&mut self) -> &mut PrimitiveManagerForThemes {
        &mut self.prim_man
    }
    pub fn set_size(&mut self, size: Vector2D<f32>) {
        self.size = size;
    }
}

impl Primitive for CommonWidget {
    fn class_name() -> &'static str {
        "CommonWidget"
    }
    // TODO change to macro
    fn class(&self) -> &'static str {
        Self::class_name()
    }
    fn nid(&self) -> usize {
        self.nid
    }
    fn set_nid(&mut self, nid: usize) {
        self.nid = nid
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
    fn size(&mut self) -> &Vector2D<f32> {
        &self.size
    }
}

impl PrivatePrimitiveMethods for CommonWidget {
    fn update_geometry(&mut self) {
        self.geometry = Geometry::new_from_prim_man(&self.class, &mut self.prim_man);
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

impl HasEvents for CommonWidget {
    fn event_mouse_button_down(&self) -> Arc<MouseButtonDownCallback> {
        self.event_mouse_button_down.clone_callback()
    }
    fn set_event_mouse_button_down(&mut self, callback: MouseButtonDownCallback) {
        self.event_mouse_button_down = MouseButtonDown {
            callback: Arc::new(callback),
        }
    }
    fn event_key_down(&self) -> Arc<KeyDownCallback> {
        self.event_key_down.clone_callback()
    }
    fn set_event_key_down(&mut self, callback: KeyDownCallback) {
        self.event_key_down = KeyDown {
            callback: Arc::new(callback),
        }
    }
}

impl Widget for CommonWidget {}
