use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::path::Path;
use std::sync::{Arc, Mutex};

use sdl2::pixels::PixelFormat;
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;

use crate::general::{Color, Polygon};
use crate::tex_man::TextureManager;

/// All Textures are lazy, because they can only be created or updated during render time.
/// https://documentation.help/SDL/thread.html
/// Note tex_creator and tex_man are references, which means it disallows multi-threaded usages.
/// All Textures are soft which means they can be cloned without allocating more GPU memory.
/// All Textures are static, once one is created it won't change. Basically at the first time it
/// gets rendered it creates the actual texture and then after than it will never change.
pub trait SoftTexture: Send {
    fn id(&self) -> usize;
    fn render(&mut self, tex_creator: &TextureCreator<WindowContext>, tex_man: &mut TextureManager)
              -> Result<Arc<Mutex<Texture>>, Box<dyn Error>>;
    fn class(&self) -> &str;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn poly(&self) -> Polygon;
}

impl Debug for dyn SoftTexture {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let id = format!("{:?}", self.id());
        let class = format!("{:?}", self.class());
        let width = format!("{:?}", self.width());
        let height = format!("{:?}", self.height());
        write!(f, "SoftTexture {{ id: {}, class: {}, width: {}, height: {} }}",
               id, class, width, height)
    }
}

/// According to rust-sdl2's Texture doc, tex.destroy() is only unsafe because you cannot destroy a
/// tex when its parent doesn't exist, so before destroying it we are checking if parent exists.
/// Note _tex_creator is a reference, which means it disallows multi-threaded usages.
/// Also we simply exit if this is not the last Arc or Mutex for this tex, otherwise the next caller
/// to lock the mutex would see undefined memory.
pub fn soft_texture_default_destroy(tex: Arc<Mutex<Texture>>,
                                    _tex_creator: &TextureCreator<WindowContext>) {
    let mutex = match Arc::try_unwrap(tex) {
        Ok(x) => x,
        Err(_) => return, // Maybe panic here
    };
    let internal_tex = match mutex.into_inner() {
        Ok(x) => x,
        Err(_) => return, // Maybe panic here
    };
    // if _tex_creator.upgrade().is_none() {
    //     return;
    // }
    unsafe {
        internal_tex.destroy()
    }
    println!("Tex destroyed!")
}

#[derive(Clone)]
pub struct BMPSoftTexture {
    id: usize,
    tex: Option<Arc<Mutex<Texture>>>,
    path: Box<Path>,
    width: u32,
    height: u32,
    poly: Polygon,
}

impl BMPSoftTexture {
    pub fn new(path: Box<Path>) -> BMPSoftTexture {
        BMPSoftTexture {
            id: 0,
            tex: None,
            path,
            width: 0,
            height: 0,
            poly: Polygon::new(),
        }
    }
}

impl SoftTexture for BMPSoftTexture {
    fn id(&self) -> usize {
        self.id
    }
    fn render(&mut self, tex_creator: &TextureCreator<WindowContext>, tex_man: &mut TextureManager)
              -> Result<Arc<Mutex<Texture>>, Box<dyn Error>> {
        if let None = self.tex {
            println!("{}", self.class());
            let surface = Surface::load_bmp(&self.path)?;
            self.width = surface.width();
            self.height = surface.height();
            let (arc_tex, id) = tex_man.reserve_from_surface(tex_creator, surface)?;
            self.id = id;
            self.poly = Polygon::new_for_rect_texture(Rect::new(0, 0, self.width, self.height),
                                                      255);
            self.tex = Some(arc_tex);
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
    fn poly(&self) -> Polygon {
        self.poly.clone()
    }
}

/// SDL_Texture has no Send implemented, and shouldn't because SDL2 doesn't allow to render in a
/// different thread. that's why render() will only be called during render time, and at that moment
/// a TextureCreator and a TextureManager are sent, so it becomes safe.
unsafe impl Send for BMPSoftTexture {}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct AlphaSoftTexture {
    id: usize,
    tex: Option<Arc<Mutex<Texture>>>,
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

        AlphaSoftTexture {
            id: 0,
            tex: None,
            raw_data,
            width,
            height,
            color,
            poly: Polygon::new(),
        }
    }
    fn update_texture_from_lazy_alpha(
        rect: &Rect,
        texture: &mut Texture,
        raw_data: &Vec<u8>,
        color: &Color,
    ) -> Result<(), Box<dyn Error>> {
        println!("{}()", stringify!(update_texture_from_lazy_alpha));
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
        let mut new_data: Vec<u8> = vec![];
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
              -> Result<Arc<Mutex<Texture>>, Box<dyn Error>> {
        if let None = self.tex {
            println!("{}", self.class());
            let (arc_tex, id) = tex_man.reserve(tex_creator, self.width, self.height)?;
            {
                let mut tex = arc_tex.lock().unwrap();
                tex.set_blend_mode(BlendMode::Blend);
                Self::update_texture_from_lazy_alpha(&Rect::new(0, 0, self.width, self.height),
                                                     &mut tex,
                                                     &self.raw_data,
                                                     &self.color)?;
            }
            self.id = id;
            self.poly = Polygon::new_for_rect_texture(Rect::new(0, 0, self.width, self.height),
                                                      self.color.a);
            self.tex = Some(arc_tex.clone());
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
    fn poly(&self) -> Polygon {
        self.poly.clone()
    }
}

unsafe impl Send for AlphaSoftTexture {}
