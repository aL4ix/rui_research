use std::cell::RefCell;
use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use log::debug;
use sdl2::pixels::{PixelFormat, PixelFormatEnum};
use sdl2::render::{BlendMode, Texture, TextureCreator};
use sdl2::video::WindowContext;

use crate::general::{Color, Polygon, Rect};
use crate::texture::{SoftTexture, TextureManager};

pub struct AlphaSoftTexture {
    id: usize,
    tex: Option<Rc<RefCell<Texture>>>,
    raw_data: Vec<u8>,
    width: u32,
    height: u32,
    color: Color,
    poly: Polygon,
}

impl AlphaSoftTexture {
    pub fn new(raw_data: Vec<u8>, width: u32, height: u32, color: Color) -> AlphaSoftTexture {
        if width == 0 || height == 0 {
            panic!("Texture dimensions cannot be zero")
        }

        let alpha = color.a();
        AlphaSoftTexture {
            id: 0,
            tex: None,
            raw_data,
            width,
            height,
            color,
            poly: Polygon::new_rect_for_texture(Rect::new(0, 0, width, height), alpha),
        }
    }
    fn update_texture_from_alpha(
        rect: &Rect<u32>,
        texture: &mut Texture,
        raw_data: &Vec<u8>,
        color: &Color,
    ) -> Result<(), Box<dyn Error>> {
        debug!("update_texture_from_alpha()");
        let format_enum = texture.query().format;
        let bytes_per_pixel = format_enum.byte_size_per_pixel();
        let pitch = bytes_per_pixel * rect.width() as usize;
        let pixel_format = PixelFormat::try_from(format_enum)?;
        let mut sdl_color = sdl2::pixels::Color {
            r: color.r(),
            g: color.g(),
            b: color.b(),
            a: 0,
        };
        let mut new_data: Vec<u8> = Vec::with_capacity(raw_data.len() * 4);
        for alpha in raw_data {
            sdl_color.a = *alpha;
            let native = sdl_color.to_u32(&pixel_format).to_ne_bytes();
            new_data.extend_from_slice(&native);
        }
        texture.update(*rect, &new_data, pitch)?;
        Ok(())
    }
}

impl SoftTexture for AlphaSoftTexture {
    fn id(&self) -> usize {
        self.id
    }
    fn render(&mut self, tex_creator: &TextureCreator<WindowContext>, tex_man: &mut TextureManager)
              -> Result<Rc<RefCell<Texture>>, Box<dyn Error>> {
        if self.tex.is_none() {
            debug!("{}", self.class());
            let (rc_tex, id) = tex_man.reserve(tex_creator, self.width,
                                               self.height, PixelFormatEnum::RGBA32)?;
            {
                let mut tex = rc_tex.borrow_mut();
                tex.set_blend_mode(BlendMode::Blend);
                Self::update_texture_from_alpha(&Rect::new(0, 0, self.width, self.height),
                                                &mut tex,
                                                &self.raw_data,
                                                &self.color)?;
            }
            self.id = id;
            self.poly = Polygon::new_rect_for_texture(Rect::new(0, 0, self.width, self.height),
                                                      self.color.a());
            self.tex = Some(rc_tex);
            self.raw_data = vec![]; // freeing raw_data from RAM since it could be large
        }
        match &self.tex {
            None => Err(Box::from("No texture was rendered/created!")),
            Some(tex) => Ok(tex.clone())
        }
    }
    fn class(&self) -> &str {
        stringify!(TextSoftTexture)
    }
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn poly(&self) -> &Polygon {
        &self.poly
    }
}

impl Debug for AlphaSoftTexture {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_dyn(f)
    }
}

unsafe impl Send for AlphaSoftTexture {}
