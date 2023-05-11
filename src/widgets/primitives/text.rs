use std::fmt::Debug;
use std::sync::{Arc, Mutex};

use glyph_brush::ab_glyph::{Font, FontArc, ScaleFont};
use log::debug;

use crate::general::{Color, Geometry, Size2D, Vector2D};
use crate::texture::{AlphaSoftTexture, SoftTexture};
use crate::widgets::primitives::private::PrivatePrimitiveMethods;
use crate::widgets::Primitive;

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
    /// An *id* of zero means it will be set to an automatic value when adding it to a window
    pub fn new(id: usize, text: &str, font_size: f32, font: FontArc, color: Color) -> Text {
        let (arc_tex, geometry, size) =
            Self::get_tex_geometry_and_size(text, font_size, font.clone(), color.clone());
        Text {
            id,
            tex: arc_tex,
            geometry,
            needs_update: false,
            text: text.to_string(),
            font_size,
            font,
            color,
            position: Default::default(),
            needs_translation: true,
            translated_geometry: Default::default(),
            size,
        }
    }
    fn get_tex_geometry_and_size(
        text: &str,
        font_size: f32,
        font: FontArc,
        color: Color,
    ) -> (Arc<Mutex<AlphaSoftTexture>>, Geometry, Vector2D<f32>) {
        let (raw_data, width, height) = Self::text_to_alpha_data(text, font_size, font);
        let tex = AlphaSoftTexture::new(raw_data, width, height, color);
        let poly = tex.poly().clone();
        let tex = Arc::new(Mutex::new(tex));
        let geometry = Geometry::new_for_texture("Text", tex.clone(), poly);
        (tex, geometry, Vector2D::new(width as f32, height as f32))
    }
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
        self.needs_update = true;
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

impl PrivatePrimitiveMethods for Text {
    fn update_geometry(&mut self) {
        let (tex, geometry, size) = Text::get_tex_geometry_and_size(
            &self.text,
            self.font_size,
            self.font.clone(),
            self.color.clone(),
        );
        self.tex = tex;
        self.geometry = geometry;
        self.size = size;
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

impl Primitive for Text {
    fn class_name() -> &'static str
    where
        Self: Sized,
    {
        "Text"
    }
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
