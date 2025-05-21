use std::fmt::Debug;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::general::{Geometry, Vector2D};
use crate::texture::{RAMSoftTexture, SoftTexture};
use crate::widgets::primitives::private::PrivatePrimitiveMethods;
use crate::widgets::Primitive;

#[derive(Debug)]
pub struct Bitmap {
    id: usize,
    arc_tex: Arc<Mutex<dyn SoftTexture>>,
    geometry: Geometry,
    needs_update: bool,
    position: Vector2D<f32>,
    needs_translation: bool,
    translated_geometry: Geometry,
    size: Vector2D<f32>,
}

impl Bitmap {
    /// An *id* of zero means it will be set to an automatic value when adding it to a window
    pub fn from_bmp(id: usize, path: Box<Path>) -> Result<Bitmap, String> {
        let tex = RAMSoftTexture::from_bmp(path)?;
        let size = Vector2D::new(tex.width() as f32, tex.height() as f32);
        let poly = tex.poly().clone();
        let arc_tex = Arc::new(Mutex::new(tex));
        Ok(Bitmap {
            id,
            arc_tex: arc_tex.clone(),
            geometry: Geometry::new_for_texture("Bitmap", arc_tex, poly),
            needs_update: false,
            position: Default::default(),
            needs_translation: true,
            translated_geometry: Default::default(),
            size,
        })
    }
}

impl PrivatePrimitiveMethods for Bitmap {
    fn update_geometry(&mut self) {
        let poly = self.arc_tex.lock().expect("bitmap:PrivatePrimitiveMethods:update_geometry").poly().clone(); // Should we save poly instead?
        self.geometry = Geometry::new_for_texture("Bitmap", self.arc_tex.clone(), poly);
    }
    fn needs_update(&self) -> bool {
        self.needs_update
    }
    fn set_needs_update(&mut self, needs_update: bool) {
        self.needs_update = needs_update;
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
        self.translated_geometry = translated_geometry;
    }
    fn clone_translated_geometry(&self) -> Geometry {
        self.translated_geometry.clone()
    }
}

impl Primitive for Bitmap {
    fn class_name() -> &'static str
    where
        Self: Sized,
    {
        "Bitmap"
    }
    // TODO change to macro
    fn class(&self) -> &'static str {
        Self::class_name()
    }
    fn nid(&self) -> usize {
        self.id
    }
    fn set_nid(&mut self, id: usize) {
        self.id = id;
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
        self.position = position;
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
