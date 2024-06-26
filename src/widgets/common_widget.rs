use std::sync::Arc;

use crate::general::{Geometry, Vector2D};
use crate::widgets::events::{Event, MouseButtonDown, MouseButtonDownCallback};
use crate::widgets::primitives::private::PrivatePrimitiveMethods;
use crate::widgets::{Primitive, Widget};

#[derive(Debug)]
pub struct CommonWidget {
    id: usize,
    position: Vector2D<f32>,
    size: Vector2D<f32>,
    geometry: Geometry,
    needs_update: bool,
    needs_translation: bool,
    translated_geometry: Geometry,
    event_mouse_button_down: MouseButtonDown,
    primitives: Vec<Box<dyn Primitive>>,
    class: String,
}

impl CommonWidget {
    pub fn new(
        id: usize,
        class: &str,
        mut primitives: Vec<Box<dyn Primitive>>,
        size: Vector2D<f32>,
    ) -> CommonWidget {
        let geometry = Geometry::new_from_primitives(class, &mut primitives);
        CommonWidget {
            id,
            position: Default::default(),
            size,
            geometry,
            needs_update: false,
            needs_translation: true,
            translated_geometry: Default::default(),
            event_mouse_button_down: Default::default(),
            primitives,
            class: class.to_string(),
        }
    }
    pub fn get_primitive_by_index_mut(&mut self, index: usize) -> &mut Box<dyn Primitive> {
        self.primitives.get_mut(index).unwrap()
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

impl PrivatePrimitiveMethods for CommonWidget {
    fn update_geometry(&mut self) {
        self.geometry = Geometry::new_from_primitives(&self.class, &mut self.primitives);
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

impl Widget for CommonWidget {
    fn event_mouse_button_down(&self) -> Arc<MouseButtonDownCallback> {
        self.event_mouse_button_down.clone_callback()
    }
    fn set_event_mouse_button_down(&mut self, callback: MouseButtonDownCallback) {
        self.event_mouse_button_down = MouseButtonDown {
            callback: Arc::new(callback),
        }
    }
}
