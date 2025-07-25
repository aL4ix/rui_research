use std::fmt::Debug;

use crate::general::{Color, Geometry, Polygon, TexturedPolygon, Vector2D};
use crate::widgets::primitives::private::PrivatePrimitiveMethods;
use crate::widgets::primitives::Primitive;

#[derive(Debug)]
pub struct Shape {
    nid: usize,
    poly: Polygon,
    needs_update: bool,
    geometry: Geometry,
    position: Vector2D<f32>,
    needs_translation: bool,
    translated_geometry: Geometry,
    size: Vector2D<f32>,
}

impl Shape {
    fn new(nid: usize, size: Vector2D<f32>, poly: Polygon) -> Shape {
        let position = Default::default();
        Shape {
            nid,
            poly: poly.clone(),
            needs_update: false,
            geometry: Self::geometry_out_of_poly(poly),
            position,
            needs_translation: true,
            translated_geometry: Default::default(),
            size,
        }
    }
    pub fn new_square(nid: usize, size: Vector2D<f32>, radius: i32, color: Color) -> Shape {
        let poly = Polygon::new_square(size.clone(), radius as f32, color);
        Self::new(nid, size, poly)
    }
    #[allow(dead_code)]
    pub fn new_reg_poly(nid: usize, size: Vector2D<f32>, sides: u32, rotate: f32) -> Shape {
        let poly = Polygon::new_reg_poly(size.clone(), sides, rotate);
        Self::new(nid, size, poly)
    }
    pub fn geometry_out_of_poly(poly: Polygon) -> Geometry {
        Geometry {
            _class: "Shape".to_string(),
            polygons: vec![TexturedPolygon { poly, tex: None }],
        }
    }
}

impl PrivatePrimitiveMethods for Shape {
    fn update_geometry(&mut self) {
        self.geometry = Shape::geometry_out_of_poly(self.poly.clone());
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

impl Primitive for Shape {
    fn class_name() -> &'static str
    where
        Self: Sized,
    {
        "Shape"
    }
    // TODO change to macro
    fn class(&self) -> &'static str {
        Self::class_name()
    }
    fn wid(&self) -> usize {
        self.nid
    }
    fn set_wid(&mut self, nid: usize) {
        self.nid = nid;
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
    fn size(&mut self) -> &Vector2D<f32> {
        &self.size
    }
}
