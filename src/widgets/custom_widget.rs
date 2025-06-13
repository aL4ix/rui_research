use std::error::Error;
use std::fmt::Debug;
use std::sync::Arc;

use crate::general::{Geometry, Vector2D};
use crate::widgets::events::{
    Event, KeyDown, KeyDownCallback, MouseButtonDown, MouseButtonDownCallback,
};
use crate::widgets::primitives::private::PrivatePrimitiveMethods;
use crate::widgets::primitives::Primitive;
use crate::widgets::themes::StyleMaster;

use super::events::HasEvents;
use super::Widget;

#[derive(Debug)]
pub struct CustomWidget {
    child: Vec<Box<dyn Widget>>,
    nid: usize,
    position: Vector2D<f32>,
    size: Vector2D<f32>,
    _style_master: Arc<StyleMaster>,
    event_mouse_button_down: MouseButtonDown,
    event_key_down: KeyDown,
    translated_geometry: Geometry,
}

impl CustomWidget {
    pub fn new(nid: usize, _style_master: Arc<StyleMaster>) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            child: Default::default(),
            nid,
            position: Default::default(),
            size: Default::default(),
            _style_master,
            event_mouse_button_down: Default::default(),
            event_key_down: Default::default(),
            translated_geometry: Default::default(),
        })
    }
    pub fn add_widget<T: Widget>(&mut self, widget: T) {
        self.child.push(Box::new(widget));
    }
}

impl Primitive for CustomWidget {
    fn class_name() -> &'static str {
        "CustomWidget"
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
    fn translate_geometry(&mut self) -> Geometry {
        let mut geometries = Vec::with_capacity(self.child.len());
        for dyn_widget in &mut self.child {
            let geometry = if dyn_widget.needs_translation() {
                dyn_widget.translate_geometry()
            } else {
                dyn_widget.clone_translated_geometry()
            };
            geometries.push(geometry);
        }
        self.translated_geometry = Geometry::new_from_geometries(Self::class_name(), geometries);
        self.translated_geometry.clone()
    }
}

impl PrivatePrimitiveMethods for CustomWidget {
    fn update_geometry(&mut self) {
        for dyn_widget in &mut self.child {
            if dyn_widget.needs_update() {
                dyn_widget.update_geometry();
            }
        }
    }
    fn needs_update(&self) -> bool {
        for dyn_widget in &self.child {
            if dyn_widget.needs_update() {
                return true;
            }
        }
        false
    }
    fn set_needs_update(&mut self, needs_update: bool) {
        for dyn_widget in &mut self.child {
            dyn_widget.set_needs_update(needs_update);
        }
    }
    fn needs_translation(&self) -> bool {
        for dyn_widget in &self.child {
            if dyn_widget.needs_translation() {
                return true;
            }
        }
        false
    }
    fn set_needs_translation(&mut self, needs_translation: bool) {
        for dyn_widget in &mut self.child {
            dyn_widget.set_needs_translation(needs_translation);
        }
    }
    fn clone_geometry(&self) -> Geometry {
        let mut geometries = Vec::with_capacity(self.child.len());
        for dyn_widget in &self.child {
            geometries.push(dyn_widget.clone_geometry());
        }
        Geometry::new_from_geometries(Self::class_name(), geometries)
    }

    fn set_translated_geometry(&mut self, _translated_geometry: Geometry) {
        panic!("set_translated_geometry This should have been called!");
    }
    fn clone_translated_geometry(&self) -> Geometry {
        self.translated_geometry.clone()
    }
}

impl HasEvents for CustomWidget {
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

impl Widget for CustomWidget {}
