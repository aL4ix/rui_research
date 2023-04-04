use std::cell::RefCell;
use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::path::Path;
use std::rc::Rc;

use log::{debug};
use sdl2::pixels::{PixelFormat, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;

use crate::general::{Color, Polygon};
use crate::tex_man::TextureManager;

/// All Textures are lazy, because they can only be created or updated during render time.
/// https://documentation.help/SDL/thread.html
/// Note tex_creator and tex_man are references, which means it disallows multi-threaded usages.
/// All Textures are static, once one is created it won't change. Basically at the first time it
/// gets rendered it creates the actual texture and then after than it will never change.
pub trait SoftTexture: Send {
    fn id(&self) -> usize;
    fn render(&mut self, tex_creator: &TextureCreator<WindowContext>, tex_man: &mut TextureManager)
              -> Result<Rc<RefCell<Texture>>, Box<dyn Error>>;
    fn class(&self) -> &str;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn poly(&self) -> &Polygon;
    fn fmt_dyn(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let id = format!("{:?}", self.id());
        let class = format!("{:?}", self.class());
        let width = format!("{:?}", self.width());
        let height = format!("{:?}", self.height());
        let poly = format!("{:?}", self.poly());
        write!(f, "SoftTexture {{ id: {}, class: {}, width: {}, height: {}, poly: {} }}",
               id, class, width, height, poly)
    }
}

impl Debug for dyn SoftTexture {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_dyn(f)
    }
}

/// According to rust-sdl2's Texture doc, tex.destroy() is only unsafe because you cannot destroy a
/// tex when its parent doesn't exist, so before destroying it we are checking if parent exists.
/// Note _tex_creator is a reference, which means it disallows multi-threaded usages.
/// Also we simply exit if this is not the last Arc or Mutex for this tex, otherwise the next caller
/// to lock the mutex would see undefined memory.
pub fn soft_texture_default_destroy(tex: Rc<RefCell<Texture>>,
                                    _tex_creator: &TextureCreator<WindowContext>) {
    // TODO integrate to TexMan
    let refcell = match Rc::try_unwrap(tex) {
        Ok(x) => x,
        Err(_) => return, // Maybe panic here
    };
    let internal_tex = refcell.into_inner();
    // if _tex_creator.upgrade().is_none() {
    //     return;
    // }
    unsafe {
        internal_tex.destroy()
    }
    debug!("Tex destroyed!");
}

pub struct RAMSoftTexture {
    id: usize,
    tex: Option<Rc<RefCell<Texture>>>,
    width: u32,
    height: u32,
    poly: Polygon,
    raw_data: Vec<u8>,
    pitch: u32,
    pixel_format: PixelFormatEnum,
}

impl RAMSoftTexture {
    pub fn from_bmp(path: Box<Path>) -> Result<RAMSoftTexture, String> {
        let surface = Surface::load_bmp(path)?;
        let raw_data = surface.with_lock(|x: &[u8]| Vec::from(x));
        let width = surface.width();
        let height = surface.height();
        let pitch = surface.pitch();
        let pixel_format = surface.pixel_format_enum();
        Ok(RAMSoftTexture {
            id: 0,
            tex: None,
            width,
            height,
            poly: Polygon::new_for_rect_texture(Rect::new(0, 0, width, height), 255),
            raw_data,
            pitch,
            pixel_format,
        })
    }
}

impl SoftTexture for RAMSoftTexture {
    fn id(&self) -> usize {
        self.id
    }
    fn render(&mut self, tex_creator: &TextureCreator<WindowContext>, tex_man: &mut TextureManager)
              -> Result<Rc<RefCell<Texture>>, Box<dyn Error>> {
        if self.tex.is_none() {
            debug!("{}", self.class());
            let (rc_tex, id) = tex_man.reserve(tex_creator, self.width,
                                               self.height, self.pixel_format)?;
            {
                let mut guard = rc_tex.borrow_mut();
                guard.update(None, &self.raw_data, self.pitch as usize)?;
            }
            self.id = id;
            self.poly = Polygon::new_for_rect_texture(Rect::new(0, 0, self.width, self.height),
                                                      255);
            self.tex = Some(rc_tex);
            self.raw_data = vec![]; // Removing raw_data since it could be large
        }
        match &self.tex {
            None => Err(Box::from("No texture was rendered/created!")),
            Some(tex) => Ok(tex.clone())
        }
    }
    fn class(&self) -> &str {
        stringify!(BMPSoftTexture)
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

/// SDL_Texture has no Send implemented, and shouldn't because SDL2 doesn't allow to render in a
/// different thread. that's why render() will only be called during render time, and at that moment
/// a TextureCreator and a TextureManager are sent, so it becomes safe.
unsafe impl Send for RAMSoftTexture {}

////////////////////////////////////////////////////////////////////////////////////////////////////

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

        let alpha = color.a;
        AlphaSoftTexture {
            id: 0,
            tex: None,
            raw_data,
            width,
            height,
            color,
            poly: Polygon::new_for_rect_texture(Rect::new(0, 0, width, height), alpha),
        }
    }
    fn update_texture_from_alpha(
        rect: &Rect,
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
            r: color.r,
            g: color.g,
            b: color.b,
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
            self.poly = Polygon::new_for_rect_texture(Rect::new(0, 0, self.width, self.height),
                                                      self.color.a);
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
