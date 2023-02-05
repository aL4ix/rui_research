use std::error::Error;
use std::sync::{Arc, Mutex};

use sdl2::render::TextureCreator;
use sdl2::surface::Surface;
use sdl2::video::WindowContext;

/// All Textures are lazy, because they can only be created or updated during render time.
pub trait Texture {
    fn render(&mut self, tex_creator: Arc<TextureCreator<WindowContext>>) -> Result<&sdl2::render::Texture, Box<dyn Error>>;
    //fn delete()
}

pub struct NeverChangingTexture {
    needs_update: bool,
    path: Box<std::path::Path>,
    tex: Option<sdl2::render::Texture>,
}

impl NeverChangingTexture {
    pub fn new(path: Box<std::path::Path>) -> NeverChangingTexture {
        NeverChangingTexture {
            needs_update: true,
            path,
            tex: None,
        }
    }
}

impl Texture for NeverChangingTexture {
    fn render(&mut self, tex_creator: Arc<TextureCreator<WindowContext>>) -> Result<&sdl2::render::Texture, Box<dyn Error>> {
        if self.needs_update {
            let surface = Surface::load_bmp(&self.path)?;
            self.tex = Some(tex_creator.create_texture_from_surface(surface)?);
        }
        match &self.tex {
            None => Err(Box::from("")),
            Some(tex) => Ok(tex)
        }
    }
}

/// SDL_Texture as no Send implemented, and shouldn't because SDL2 doesn't allow to render in a
/// different thread. that's why render() will only be called during render time, and at that moment
/// a TextureCreator is sent, so it becomes safe.
unsafe impl Send for NeverChangingTexture {}

struct Polygon {
    tex: Option<Arc<Mutex<dyn Texture>>>,
}

struct Text {
    tex: Arc<Mutex<dyn Texture>>,
}

impl Text {
    fn build(&self) -> Polygon {
        Polygon {
            tex: Some(self.tex.clone())
        }
    }
}

struct Image {
    tex: Arc<Mutex<dyn Texture>>,
}

impl Text {}