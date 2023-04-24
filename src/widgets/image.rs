use std::fmt::Debug;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::general::{Geometry, Polygon, TexturedPolygon, Vector2D};
use crate::texture::{RAMSoftTexture, SoftTexture};
use crate::widgets::Primitive;
use crate::widgets::primitive::private::PrivatePrimitiveMethods;

#[derive(Debug)]
pub struct Image {
    id: usize,
    tex: Arc<Mutex<dyn SoftTexture>>,
    geometry: Geometry,
    needs_update: bool,
    position: Vector2D<f32>,
    needs_translation: bool,
    translated_geometry: Geometry,
    size: Vector2D<f32>,
}

impl Image {
    /// An *id* of zero means it will be set to an automatic value when adding it to a window
    pub fn from_bmp(id: usize, path: Box<Path>) -> Result<Image, String> {
        let tex = RAMSoftTexture::from_bmp(path)?;
        let size = Vector2D::new(tex.width() as f32, tex.height() as f32);
        let poly = tex.poly().clone();
        let arc_tex = Arc::new(Mutex::new(tex));
        Ok(Image {
            id,
            tex: arc_tex.clone(),
            geometry: Geometry::new_for_texture("Image", arc_tex, poly),
            needs_update: false,
            position: Default::default(),
            needs_translation: true,
            translated_geometry: Default::default(),
            size,
        })
    }
}

impl PrivatePrimitiveMethods for Image {
    fn update_geometry(&mut self) {
        self.geometry.polygons = vec![TexturedPolygon {
            poly: Polygon { vers: vec![], inds: vec![] },
            tex: Some(self.tex.clone()),
        }];
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

impl Primitive for Image {
    fn id(&self) -> usize {
        self.id
    }
    fn set_id(&mut self, id: usize) {
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
