use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;
use log::{debug};

use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use crate::texture::soft_texture_default_destroy;

pub struct TextureManager {
    textures: HashMap<usize, Rc<RefCell<Texture>>>,
    last_id: usize,
}

impl TextureManager {
    pub fn new() -> TextureManager {
        TextureManager {
            textures: Default::default(),
            last_id: 0,
        }
    }
    fn push(&mut self, tex: &Rc<RefCell<Texture>>) -> usize {
        self.last_id += 1;
        self.textures.insert(self.last_id, tex.clone());
        debug!("Created tex: {}", self.last_id);
        self.last_id
    }
    // pub fn reserve_from_surface(&mut self, tex_creator: &TextureCreator<WindowContext>, surface: Surface)
    //                             -> Result<(Arc<Mutex<sdl2::render::Texture>>, usize), Box<dyn Error>> {
    //     let arc = Arc::new(Mutex::new(tex_creator.create_texture_from_surface(surface)?));
    //     let id = self.push(&arc);
    //     Ok((arc, id))
    // }
    pub fn reserve(&mut self, tex_creator: &TextureCreator<WindowContext>, width: u32, height: u32,
                   format: PixelFormatEnum)
                   -> Result<(Rc<RefCell<Texture>>, usize), Box<dyn Error>> {
        let tex = tex_creator.create_texture_static(format, width, height)?;
        let rc = Rc::new(RefCell::new(tex));
        let id = self.push(&rc);
        Ok((rc, id))
    }
    pub fn garbage_collect(&mut self, tex_creator: TextureCreator<WindowContext>) {
        let mut garbage = vec![];
        for (id, tex) in &self.textures {
            if Rc::strong_count(tex) == 1 {
                garbage.push(*id);
            }
        }
        for id in garbage {
            debug!("Killing tex: {}", id);
            let tex = self.textures.remove(&id).unwrap();
            soft_texture_default_destroy(tex, &tex_creator);
        }
    }
}