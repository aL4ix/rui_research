use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};

use sdl2::pixels::PixelFormatEnum;
use sdl2::render::TextureCreator;
use sdl2::surface::Surface;
use sdl2::video::WindowContext;

use crate::texture::soft_texture_default_destroy;

pub struct TextureManager {
    texs: HashMap<usize, Arc<Mutex<sdl2::render::Texture>>>,
    last_id: usize,
}

impl TextureManager {
    pub fn new() -> TextureManager {
        TextureManager {
            texs: Default::default(),
            last_id: 0,
        }
    }
    fn push(&mut self, tex: &Arc<Mutex<sdl2::render::Texture>>) -> usize {
        self.last_id += 1;
        self.texs.insert(self.last_id, tex.clone());
        println!("Created tex: {}", self.last_id);
        self.last_id
    }
    pub fn reserve_from_surface(&mut self, tex_creator: &TextureCreator<WindowContext>, surface: Surface)
                                -> Result<(Arc<Mutex<sdl2::render::Texture>>, usize), Box<dyn Error>> {
        let arc = Arc::new(Mutex::new(tex_creator.create_texture_from_surface(surface)?));
        let id = self.push(&arc);
        Ok((arc, id))
    }
    pub fn reserve(&mut self, tex_creator: &TextureCreator<WindowContext>, width: u32, height: u32)
                   -> Result<(Arc<Mutex<sdl2::render::Texture>>, usize), Box<dyn Error>> {
        let tex = tex_creator.create_texture_static(PixelFormatEnum::RGBA32, width, height)?;
        let arc = Arc::new(Mutex::new(tex));
        let id = self.push(&arc);
        Ok((arc, id))
    }
    pub fn garbage_collect(&mut self, tex_creator: TextureCreator<WindowContext>) {
        let mut garbage = vec![];
        for (id, tex) in &self.texs {
            if Arc::strong_count(&tex) == 1 {
                garbage.push(*id);
            }
        }
        for id in garbage {
            println!("Killing tex: {}", id);
            let tex = self.texs.remove(&id).unwrap();
            soft_texture_default_destroy(Some(tex), &tex_creator);
        }
    }
}