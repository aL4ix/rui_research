use std::error::Error;
use std::fmt::Debug;
use std::path::Path;
use std::sync::{Arc, Mutex};

use glyph_brush::ab_glyph::{Font, FontArc, ScaleFont};
use mopa::{Any, mopafy};

use crate::general::{Body, Color, Polygon, Size2D, TexturedPolygon};
use crate::texture::{AlphaSoftTexture, RAMSoftTexture, SoftTexture};
use crate::window::Window;

pub trait Widget: Any + Debug + Send {
    // TODO global ids
    fn id(&self) -> usize;
    fn build(&mut self) -> Body;
}

mopafy!(Widget);

#[derive(Debug)]
pub struct Image {
    id: usize,
    tex: Arc<Mutex<dyn SoftTexture>>,
    body: Body,
    needs_update: bool,
}

impl Image {
    pub fn from_bmp(id: usize, path: Box<Path>) -> Result<Image, String> {
        let tex = RAMSoftTexture::from_bmp(path)?;
        let poly = tex.poly().clone();
        let arc_tex = Arc::new(Mutex::new(tex));
        Ok(Image {
            id,
            tex: arc_tex.clone(),
            body: Body::new_for_texture("Image", arc_tex, poly),
            needs_update: false,
        })
    }
}

impl Widget for Image {
    fn id(&self) -> usize {
        self.id
    }
    fn build(&mut self) -> Body {
        if self.needs_update {
            self.body.polygons = vec![TexturedPolygon {
                poly: Polygon { vers: vec![], inds: vec![] },
                tex: Some(self.tex.clone()),
            }];
            self.needs_update = false;
        }
        self.body.clone()
    }
}

#[derive(Debug)]
pub struct Text {
    id: usize,
    tex: Arc<Mutex<AlphaSoftTexture>>,
    body: Body,
    needs_update: bool,
    text: String,
    font_size: f32,
    font: FontArc,
    color: Color,
}

impl Text {
    pub fn new(id: usize, text: &str, font_size: f32, font: FontArc, color: Color) -> Text {
        let (tex, body) = Self::get_tex_and_body(text, font_size,
                                                 font.clone(), color.clone());
        Text {
            id,
            tex,
            body,
            needs_update: false,
            text: text.to_string(),
            font_size,
            font,
            color,
        }
    }
    fn get_tex_and_body(text: &str, font_size: f32, font: FontArc, color: Color)
                        -> (Arc<Mutex<AlphaSoftTexture>>, Body) {
        let (raw_data, width, height) = Self::text_to_alpha_data(text,
                                                                 font_size, font);
        let tex = AlphaSoftTexture::new(raw_data, width, height, color);
        let poly = tex.poly().clone();
        let tex = Arc::new(Mutex::new(tex));
        let body = Body::new_for_texture("Text", tex.clone(), poly);
        (tex, body)
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
        println!("{}() {:?}", stringify!(text_to_alpha), bounds);
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
            // println!(
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
                    //println!("{} {} {}", x, y, c)
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

impl Widget for Text {
    fn id(&self) -> usize {
        self.id
    }
    fn build(&mut self) -> Body {
        if self.needs_update {
            let (tex, body) = Text::get_tex_and_body(&self.text,
                                                     self.font_size, self.font.clone(),
                                                     self.color.clone());
            self.tex = tex;
            self.body = body;
            self.needs_update = false;
        }
        self.body.clone()
    }
}
