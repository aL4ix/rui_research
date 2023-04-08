use std::error::Error;
use std::fmt::Debug;
use std::path::Path;
use std::sync::{Arc, Mutex};

use glyph_brush::ab_glyph::{Font, FontArc, ScaleFont};
use log::debug;
use mopa::{Any, mopafy};

use crate::general::{Color, Geometry, Polygon, Size2D, TexturedPolygon, Vector2D};
use crate::texture::{AlphaSoftTexture, RAMSoftTexture, SoftTexture};
use crate::window::Window;

pub(in crate::widgets) mod private {
    use crate::general::Geometry;

    pub trait PrivateWidgetMethods {
        fn update_geometry(&mut self);
        fn needs_update(&self) -> bool;
        fn set_needs_update(&mut self, needs_update: bool);
        fn needs_translation(&self) -> bool;
        fn set_needs_translation(&mut self, needs_translation: bool);
        fn clone_geometry(&self) -> Geometry;
        fn set_translated_geometry(&mut self, translated_geometry: Geometry);
        fn clone_translated_geometry(&self) -> Geometry;
    }
}

pub trait Widget: Any + Debug + Send + private::PrivateWidgetMethods {
    // TODO global ids
    fn id(&self) -> usize;
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn position(&self) -> &Vector2D<f32>;
    fn set_position(&mut self, position: Vector2D<f32>);
    fn width(&self) -> f32;
    fn height(&self) -> f32;
    fn translate_geometry(&mut self) -> Geometry {
        let mut geometry = self.clone_geometry();
        geometry.translate(self.position());
        self.set_translated_geometry(geometry.clone());
        self.set_needs_translation(false);
        geometry
    }
    fn build(&mut self) -> Geometry {
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

mopafy!(Widget);

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
    pub fn from_bmp(id: usize, path: Box<Path>) -> Result<Image, String> {
        let tex = RAMSoftTexture::from_bmp(path)?;
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
            size: Default::default(),
        })
    }
}

impl private::PrivateWidgetMethods for Image {
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

impl Widget for Image {
    fn id(&self) -> usize {
        self.id
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
}

#[derive(Debug)]
pub struct Text {
    id: usize,
    tex: Arc<Mutex<AlphaSoftTexture>>,
    geometry: Geometry,
    needs_update: bool,
    text: String,
    font_size: f32,
    font: FontArc,
    color: Color,
    position: Vector2D<f32>,
    needs_translation: bool,
    translated_geometry: Geometry,
    size: Vector2D<f32>,
}

impl Text {
    pub fn new(id: usize, text: &str, font_size: f32, font: FontArc, color: Color) -> Text {
        let (tex, geometry) = Self::get_tex_and_geometry(text, font_size,
                                                         font.clone(), color.clone());
        Text {
            id,
            tex,
            geometry,
            needs_update: false,
            text: text.to_string(),
            font_size,
            font,
            color,
            position: Default::default(),
            needs_translation: true,
            translated_geometry: Default::default(),
            size: Default::default(),
        }
    }
    fn get_tex_and_geometry(text: &str, font_size: f32, font: FontArc, color: Color)
                            -> (Arc<Mutex<AlphaSoftTexture>>, Geometry) {
        let (raw_data, width, height) = Self::text_to_alpha_data(text,
                                                                 font_size, font);
        let tex = AlphaSoftTexture::new(raw_data, width, height, color);
        let poly = tex.poly().clone();
        let tex = Arc::new(Mutex::new(tex));
        let geometry = Geometry::new_for_texture("Text", tex.clone(), poly);
        (tex, geometry)
    }
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
        self.needs_update = true;
    }
    pub fn get_by_id(window: &mut Window, id: usize) -> Result<&mut Text, Box<dyn Error>> {
        if let Some(widget) = window.get_widget_by_id(id) {
            return if let Some(text) = widget.downcast_mut::<Text>() {
                Ok(text)
            } else {
                Err(Box::from("Not a Text"))
            };
        }
        Err(Box::from("Not found"))
    }
    fn text_to_alpha_data(text: &str, font_size: f32, font: FontArc) -> (Vec<u8>, u32, u32) {
        let bounds = Self::get_texture_bounds(text, font_size, font.clone());
        debug!("{}() {:?}", stringify!(text_to_alpha), bounds);
        let width = bounds.width;
        let height = bounds.height;
        let scaled_font = font.as_scaled(font_size);
        let mut accumulated_x = 0.0;
        let mut raw_data = vec![0; (width * height) as usize];
        for ch in text.chars() {
            let glyph = scaled_font.scaled_glyph(ch);
            let glyph_bounds = scaled_font.glyph_bounds(&glyph);
            let outline = scaled_font.outline_glyph(glyph);
            // let width = outline.as_ref().unwrap().px_bounds().width() as i32;
            // let height = scaled_font.height();
            // let line_gap = scaled_font.line_gap();
            let glyph_id = scaled_font.glyph_id(ch);
            // let v_advance = scaled_font.v_advance(glyph_id);
            let v_side_bearing = scaled_font.v_side_bearing(glyph_id);
            let top_space = -glyph_bounds.min.y - v_side_bearing;
            // info!(
            //     "{} {} {} {} {} {:?}",
            //     ch, width, height, v_advance, v_side_bearing, glyph_bounds
            // );

            accumulated_x += glyph_bounds.min.x;
            if let Some(q) = outline {
                q.draw(|x, y, c| {
                    /* draw pixel `(x, y)` with coverage: `c` */
                    let alpha = (255.0 * c) as u8;
                    let dx = x as usize + accumulated_x.round() as usize;
                    let dy = y as usize + top_space.round() as usize;
                    raw_data[dy * width as usize + dx] = alpha;
                    //info!("{} {} {}", x, y, c)
                });
            }

            accumulated_x += glyph_bounds.max.x;
        }
        (raw_data, width, height)
    }
    fn get_texture_bounds(text: &str, size: f32, font: FontArc) -> Size2D {
        let scaled_font = font.as_scaled(size);
        let height = scaled_font.height();
        let mut width = 0.0;
        for c in text.chars() {
            let glyph = scaled_font.scaled_glyph(c);
            width += scaled_font.glyph_bounds(&glyph).width();
        }
        Size2D {
            width: width.ceil() as u32,
            height: height.ceil() as u32,
        }
    }
}

impl private::PrivateWidgetMethods for Text {
    fn update_geometry(&mut self) {
        let (tex, geometry) = Text::get_tex_and_geometry(&self.text,
                                                         self.font_size, self.font.clone(),
                                                         self.color.clone());
        self.tex = tex;
        self.geometry = geometry;
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
        self.translated_geometry = translated_geometry;
    }
    fn clone_translated_geometry(&self) -> Geometry {
        self.translated_geometry.clone()
    }
}

impl Widget for Text {
    fn id(&self) -> usize {
        self.id
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
}

#[derive(Debug)]
pub struct Shape {
    id: usize,
    poly: Polygon,
    needs_update: bool,
    geometry: Geometry,
    position: Vector2D<f32>,
    needs_translation: bool,
    translated_geometry: Geometry,
    size: Vector2D<f32>,
}

impl Shape {
    fn new(size: Vector2D<f32>, poly: Polygon) -> Shape {
        let position = Default::default();
        Shape {
            id: 0,
            poly: poly.clone(),
            needs_update: true,
            geometry: Self::geometry_out_of_poly(poly),
            position,
            needs_translation: true,
            translated_geometry: Default::default(),
            size,
        }
    }
    pub fn new_square(size: Vector2D<f32>, radius: i32, color: Color) -> Shape {
        let poly = Polygon::new_square(size.clone(), radius as f32, color);
        Self::new(size, poly)
    }
    pub fn new_reg_poly(size: Vector2D<f32>, sides: u32, rotate: f32) -> Shape {
        let poly = Polygon::new_reg_poly(size.clone(), sides, rotate);
        Self::new(size, poly)
    }
    pub fn geometry_out_of_poly(poly: Polygon) -> Geometry {
        Geometry {
            class: "Shape".to_string(),
            polygons: vec![TexturedPolygon {
                poly,
                tex: None,
            }],
        }
    }
}

impl private::PrivateWidgetMethods for Shape {
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

impl Widget for Shape {
    fn id(&self) -> usize {
        self.id
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
}