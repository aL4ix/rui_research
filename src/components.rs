use std::path::Path;
use std::sync::{Arc, Mutex};

use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use crate::tex_man::TextureManager;
use crate::texture::{BMPSoftTexture, SoftTexture};

// struct Polygon {
//     tex: Option<Arc<Mutex<dyn SoftTexture>>>,
// }

pub trait BuilderCompo {
    fn render(&mut self, tex_creator: &TextureCreator<WindowContext>, tex_man: &mut TextureManager)
              -> Result<Arc<Mutex<Texture>>, Box<(dyn std::error::Error)>>;
}

// struct Text {
//     tex: Arc<Mutex<dyn SoftTexture>>,
// }
//
// impl Text {
//     fn build(&self) -> Polygon {
//         Polygon {
//             tex: Some(self.tex.clone())
//         }
//     }
// }

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
              -> Result<Arc<Mutex<Texture>>, Box<(dyn std::error::Error)>> {
        self.tex.render(tex_creator, tex_man)
    }
}