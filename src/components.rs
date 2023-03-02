use std::error::Error;
use std::path::Path;
use std::sync::{Arc, Mutex};

use glyph_brush::ab_glyph::{Font, FontArc, ScaleFont};
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use crate::general::{Color, Size2D};
use crate::tex_man::TextureManager;
use crate::texture::{AlphaSoftTexture, BMPSoftTexture, SoftTexture};

pub trait BuilderCompo {
    // TODO change it to return an SDLBody
    fn render(&mut self, tex_creator: &TextureCreator<WindowContext>, tex_man: &mut TextureManager)
              -> Result<Arc<Mutex<Texture>>, Box<(dyn Error)>>;
}

pub struct Image {
    tex: Box<dyn SoftTexture>,
}

impl Image {
    pub fn from_bmp(path: Box<Path>) -> Image {
        Image {
            tex: Box::from(BMPSoftTexture::new(path))
        }
    }
}

impl BuilderCompo for Image {
    fn render(&mut self, tex_creator: &TextureCreator<WindowContext>, tex_man: &mut TextureManager)
              -> Result<Arc<Mutex<Texture>>, Box<(dyn Error)>> {
        self.tex.render(tex_creator, tex_man)
    }
}

pub struct Text {
    tex: Box<AlphaSoftTexture>,
}

impl Text {
    pub fn new(text: &str, font_size: f32, font: FontArc, color: Color) -> Text {
        let (raw_data, width, height) = Self::text_to_alpha_data(text, font_size, font);
        Text {
            tex: Box::new(AlphaSoftTexture::new(raw_data, width, height, color)),
        }
    }
    fn text_to_alpha_data(text: &str, font_size: f32, font: FontArc) -> (Vec<u8>, u32, u32) {
        let bounds = Self::get_texture_bounds(&text, font_size, font.clone());
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
        // self.poly = SDLPolygon::new_for_rect_texture(Rect::new(0, 0, self.width as u32, self.height as u32));
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

impl BuilderCompo for Text {
    fn render(&mut self, tex_creator: &TextureCreator<WindowContext>, tex_man: &mut TextureManager) -> Result<Arc<Mutex<Texture>>, Box<(dyn Error)>> {
        self.tex.render(tex_creator, tex_man)
    }
}