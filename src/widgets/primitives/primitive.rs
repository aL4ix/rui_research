use std::any::Any;
use std::fmt::Debug;

use crate::general::{Geometry, Vector2D};

pub(in crate::widgets) mod private {
    use crate::general::Geometry;

    pub trait PrivatePrimitiveMethods {
        /// Updating the geometry implies re-generating the Primitive and
        fn update_geometry(&mut self);
        /// If the Primitive needs all updates, from re-generating the geometry of the Primitive itself and translation too.
        fn needs_update(&self) -> bool;
        /// Set if the Primitive needs all updates, from re-generating the geometry fot the Primitive itself and translation too.
        fn set_needs_update(&mut self, needs_update: bool);
        /// Returns if the Primitive needs a translation.
        fn needs_translation(&self) -> bool;
        fn set_needs_translation(&mut self, needs_translation: bool);
        fn clone_geometry(&self) -> Geometry;
        fn set_translated_geometry(&mut self, translated_geometry: Geometry);
        fn clone_translated_geometry(&self) -> Geometry;
    }
}

pub trait Primitive: Debug + Send + private::PrivatePrimitiveMethods + Any {
    fn class_name() -> &'static str
    where
        Self: Sized;
    fn class(&self) -> &'static str;
    fn wid(&self) -> usize;
    fn set_wid(&mut self, nid: usize);
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn position(&self) -> &Vector2D<f32>;
    fn set_position(&mut self, position: Vector2D<f32>);
    fn width(&self) -> f32;
    fn height(&self) -> f32;
    fn size(&mut self) -> &Vector2D<f32>;
    fn translate_geometry(&mut self) -> Geometry {
        let mut geometry = self.clone_geometry();
        geometry.translate(self.position());
        self.set_translated_geometry(geometry.clone());
        self.set_needs_translation(false);
        geometry
    }
    fn build_geometry(&mut self) -> Geometry {
        if self.needs_update() {
            self.update_geometry();
            self.set_needs_update(false);
            self.translate_geometry()
        } else if self.needs_translation() {
            self.translate_geometry()
        } else {
            self.clone_translated_geometry()
        }
    }
}
