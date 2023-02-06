use std::path::Path;
use std::rc::Weak;
use std::sync::{Arc, Mutex};

use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use crate::texture::{BMPSoftTexture, SoftTexture};

struct Polygon {
    tex: Option<Arc<Mutex<dyn SoftTexture>>>,
}

#[derive(Clone)]
pub struct RenderContext {
    tex_creator: Weak<TextureCreator<WindowContext>>,
}

impl RenderContext {
    pub fn new(tex_creator: Weak<TextureCreator<WindowContext>>) -> RenderContext {
        RenderContext {
            tex_creator,
        }
    }
}

pub trait BuilderCompo {
    fn render(&mut self, context: RenderContext)
              -> Result<Arc<Mutex<Texture>>, Box<(dyn std::error::Error)>>;
}

struct Text {
    tex: Arc<Mutex<dyn SoftTexture>>,
}

impl Text {
    fn build(&self) -> Polygon {
        Polygon {
            tex: Some(self.tex.clone())
        }
    }
}

pub struct Image {
    tex: Box<dyn SoftTexture>
}

impl Image {
    pub fn from_bmp(path: Box<Path>) -> Image {
        Image {
            tex: Box::from(BMPSoftTexture::new(path))
        }
    }
}

impl BuilderCompo for Image {
    fn render(&mut self, context: RenderContext)
              -> Result<Arc<Mutex<Texture>>, Box<(dyn std::error::Error)>> {
        let tex_creator = context.tex_creator;
        self.tex.render(tex_creator)
    }
}
