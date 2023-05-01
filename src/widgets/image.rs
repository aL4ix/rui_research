use std::error::Error;
use std::fmt::{Debug};
use std::path::Path;
use crate::general::{Geometry, Vector2D};
use crate::widgets::{CommonWidget, Primitive, Widget};
use crate::widgets::primitives::Bitmap;
use crate::widgets::primitives::private::PrivatePrimitiveMethods;

#[derive(Debug)]
pub struct Image {
    common: CommonWidget,
}

impl Image {
    pub fn from_bmp(id: usize, path: Box<Path>) -> Result<Image, Box<dyn Error>> {
        let bitmap = Bitmap::from_bmp(0, path)?;
        let size = bitmap.size().clone();
        Ok(Image {
            common: CommonWidget::new(id, "Image", vec![Box::new(bitmap)], size),
        })
    }
}

impl Primitive for Image {
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

impl Widget for Image {
    fn event_mouse_button_down(&mut self, x: i32, y: i32) {
        self.common.event_mouse_button_down(x, y)
    }

    fn set_event_mouse_button_down(&mut self, callback: fn(&mut dyn Widget, i32, i32)) {
        self.common.set_event_mouse_button_down(callback)
    }
}