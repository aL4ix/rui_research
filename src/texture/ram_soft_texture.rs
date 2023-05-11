use std::cell::RefCell;
use std::error::Error;
use std::path::Path;
use std::rc::Rc;

use log::debug;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;

use crate::general::{Polygon, Rect};
use crate::texture::{SoftTexture, TextureManager};

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
            poly: Polygon::new_rect_for_texture(Rect::new(0, 0, width, height), 255),
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
    fn render(
        &mut self,
        tex_creator: &TextureCreator<WindowContext>,
        tex_man: &mut TextureManager,
    ) -> Result<Rc<RefCell<Texture>>, Box<dyn Error>> {
        if self.tex.is_none() {
            debug!("{}", self.class());
            let (rc_tex, id) =
                tex_man.reserve(tex_creator, self.width, self.height, self.pixel_format)?;
            {
                let mut guard = rc_tex.borrow_mut();
                guard.update(None, &self.raw_data, self.pitch as usize)?;
            }
            self.id = id;
            self.poly =
                Polygon::new_rect_for_texture(Rect::new(0, 0, self.width, self.height), 255);
            self.tex = Some(rc_tex);
            self.raw_data = vec![]; // Removing raw_data since it could be large
        }
        match &self.tex {
            None => Err(Box::from("No texture was rendered/created!")),
            Some(tex) => Ok(tex.clone()),
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
