use std::error::Error;
use std::path::Path;
use std::sync::{Arc, Mutex};

use sdl2::render::{Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;

use crate::tex_man::TextureManager;

/// All Textures are lazy, because they can only be created or updated during render time.
/// https://documentation.help/SDL/thread.html
/// Note tex_creator and tex_man are references, which means it disallows multi-threaded usages.
/// All Textures are soft which means they can be cloned without allocating more GPU memory.
pub trait SoftTexture {
    fn id(&self) -> u32;
    fn render(&mut self, tex_creator: &TextureCreator<WindowContext>, tex_man: &mut TextureManager)
              -> Result<Arc<Mutex<Texture>>, Box<dyn Error>>;
}

/// According to rust-sdl2's Texture doc, tex.destroy() is only unsafe because you cannot destroy a
/// tex when its parent doesn't exist, so before destroying it we are checking if parent exists.
/// Note _tex_creator is a reference, which means it disallows multi-threaded usages.
/// Also we simply exit if this is not the last Arc or Mutex for this tex, otherwise the next caller
/// to lock the mutex would see undefined memory.
pub fn soft_texture_default_destroy(tex: Option<Arc<Mutex<Texture>>>,
                                    _tex_creator: &TextureCreator<WindowContext>) {
    let arc = tex.unwrap();
    let mutex = match Arc::try_unwrap(arc) {
        Ok(x) => x,
        Err(_) => return,
    };
    let internal_tex = match mutex.into_inner() {
        Ok(x) => x,
        Err(_) => return,
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
    id: u32,
    needs_update: bool,
    tex: Option<Arc<Mutex<Texture>>>,
    path: Box<Path>,
}

impl BMPSoftTexture {
    pub fn new(path: Box<Path>) -> BMPSoftTexture {
        BMPSoftTexture {
            id: 0,
            needs_update: true,
            tex: None,
            path,
        }
    }
}

impl SoftTexture for BMPSoftTexture {
    fn id(&self) -> u32 {
        self.id
    }

    fn render(&mut self, tex_creator: &TextureCreator<WindowContext>, tex_man: &mut TextureManager)
              -> Result<Arc<Mutex<Texture>>, Box<dyn Error>> {
        if self.needs_update {
            println!("Tex {}: needs_update", self.id);
            let surface = Surface::load_bmp(&self.path)?;
            let (arc_tex, id) = tex_man.reserve_from_surface(tex_creator, surface)?;
            self.id = id;
            self.tex = Some(arc_tex);
            self.needs_update = false;
        }
        match &self.tex {
            None => Err(Box::from("No texture was rendered/created!")),
            Some(tex) => Ok(tex.clone())
        }
    }
}

/// SDL_Texture has no Send implemented, and shouldn't because SDL2 doesn't allow to render in a
/// different thread. that's why render() will only be called during render time, and at that moment
/// a TextureCreator is sent, so it becomes safe.
unsafe impl Send for BMPSoftTexture {}

////////////////////////////////////////////////////////////////////////////////////////////////////

// #[derive(Clone)]
// pub struct RAMSoftTexture {
//     id: u32,
//     needs_update: bool,
//     tex: Option<Arc<Mutex<sdl2::render::Texture>>>,
//     raw_data: Vec<u8>,
// }
//
// impl RAMSoftTexture {
//     pub fn from(raw_data: Vec<u8>) -> RAMSoftTexture {
//         RAMSoftTexture {
//             id: 0,
//             needs_update: true,
//             tex: None,
//             raw_data,
//         }
//     }
//     pub fn new() -> RAMSoftTexture {
//         Self::from(vec![])
//     }
// }
//
// impl SoftTexture for RAMSoftTexture {
//     fn id(&self) -> u32 {
//         self.id
//     }
//     fn render(&mut self, tex_creator: &TextureCreator<WindowContext>, tex_man: &mut TextureManager)
//               -> Result<Arc<Mutex<Texture>>, Box<dyn Error>> {
//         todo!()
//     }
// }

// pub struct EmptySoftTexture {}
//
// impl SoftTexture for EmptySoftTexture {
//     fn id(&self) -> u32 {
//         0
//     }
//     fn render(&mut self, tex_creator: &TextureCreator<WindowContext>, tex_man: &mut TextureManager) -> Result<Arc<Mutex<Texture>>, Box<dyn Error>> {
//         todo!()
//     }
// }