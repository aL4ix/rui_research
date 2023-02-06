use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use std::rc::Weak;
use std::sync::{Arc, Mutex};

use sdl2::render::TextureCreator;
use sdl2::surface::Surface;
use sdl2::video::WindowContext;

/// All Textures are lazy, because they can only be created or updated during render time.
/// https://documentation.help/SDL/thread.html
/// Note tex_creator is not an Arc or Mutex, which means it disallows multi-threaded usages.
/// All Texture are soft which means they can be cloned without allocating more GPU memory.
pub trait SoftTexture {
    fn id(&self) -> u32;
    fn render(&mut self, tex_creator: Weak<TextureCreator<WindowContext>>)
              -> Result<Arc<Mutex<sdl2::render::Texture>>, Box<dyn Error>>;
    // fn destroy(self, tex_creator: Weak<TextureCreator<WindowContext>>);
}

/// According to rust-sdl2's Texture doc, tex.destroy() is only unsafe because you cannot destroy a
/// tex when it's parent doesn't exist, so before destroying it we are checking if parent exists.
/// Note tex_creator is not an Arc or Mutex, which means it disallows multi-threaded usages.
/// Also we simply exit if this is not the last Arc or Mutex for this tex, otherwise the next caller
/// to lock the mutex would see undefined memory.
fn soft_texture_default_destroy(tex: Option<Arc<Mutex<sdl2::render::Texture>>>,
                                tex_creator: Weak<TextureCreator<WindowContext>>) {
    let arc = tex.unwrap();
    let mutex = match Arc::try_unwrap(arc) {
        Ok(x) => x,
        Err(_) => return,
    };
    let internal_tex = match mutex.into_inner() {
        Ok(x) => x,
        Err(_) => return,
    };
    if tex_creator.upgrade().is_none() {
        return;
    }
    unsafe {
        internal_tex.destroy()
    }
    println!("Really destroyed!")
}

pub struct TextureManager {
    texs: HashMap<u32, Arc<Mutex<dyn SoftTexture>>>,
    last_id: u32,
}

impl TextureManager {
    pub fn new() -> TextureManager {
        TextureManager {
            texs: Default::default(),
            last_id: 0,
        }
    }
    pub fn reserve_from_bmp(&mut self, path: Box<Path>) -> Arc<Mutex<dyn SoftTexture>> {
        let tex = Arc::new(Mutex::new(BMPSoftTexture::new(path)));
        self.last_id += 1;
        self.texs.insert(self.last_id, tex.clone());
        tex
    }
}
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct BMPSoftTexture {
    id: u32,
    needs_update: bool,
    tex: Option<Arc<Mutex<sdl2::render::Texture>>>,
    source: Option<Weak<TextureCreator<WindowContext>>>,
    path: Box<Path>,
}

impl BMPSoftTexture {
    pub fn new(path: Box<Path>) -> BMPSoftTexture {
        BMPSoftTexture {
            id: 0,
            needs_update: true,
            tex: None,
            source: None,
            path,
        }
    }
}

impl SoftTexture for BMPSoftTexture {
    fn id(&self) -> u32 {
        self.id
    }
    fn render(&mut self, weak_tex_creator: Weak<TextureCreator<WindowContext>>)
              -> Result<Arc<Mutex<sdl2::render::Texture>>, Box<dyn Error>> {
        if self.needs_update {
            let surface = Surface::load_bmp(&self.path)?;
            let tex_creator = weak_tex_creator.upgrade()
                .ok_or("No TextureCreator was found!")?;
            self.tex = Some(Arc::new(Mutex::new(tex_creator.create_texture_from_surface(surface)?)));
        }
        match self.tex.clone() {
            None => Err(Box::from("No texture was found!")),
            Some(tex) => Ok(tex)
        }
    }
    // fn destroy(self, tex_creator: Weak<TextureCreator<WindowContext>>) {
    //     soft_texture_default_destroy(self.tex, tex_creator)
    // }
}

/// SDL_Texture has no Send implemented, and shouldn't because SDL2 doesn't allow to render in a
/// different thread. that's why render() will only be called during render time, and at that moment
/// a TextureCreator is sent, so it becomes safe.
unsafe impl Send for BMPSoftTexture {}

impl Drop for BMPSoftTexture {
    fn drop(&mut self) {
        if let Some(x) = &self.source {
            // soft_texture_default_destroy(self.tex, *x);
        }
        println!("dropped");
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct RAMSoftTexture {
    id: u32,
    needs_update: bool,
    tex: Option<Arc<Mutex<sdl2::render::Texture>>>,
    raw_data: Vec<u8>,
}

impl RAMSoftTexture {
    pub fn from(raw_data: Vec<u8>) -> RAMSoftTexture {
        RAMSoftTexture {
            id: 0,
            needs_update: true,
            tex: None,
            raw_data,
        }
    }
    pub fn new() -> RAMSoftTexture {
        Self::from(vec![])
    }
}

impl SoftTexture for RAMSoftTexture {
    fn id(&self) -> u32 {
        self.id
    }
    fn render(&mut self, tex_creator: Weak<TextureCreator<WindowContext>>)
              -> Result<Arc<Mutex<sdl2::render::Texture>>, Box<dyn Error>> {
        todo!()
    }
    // fn destroy(self, tex_creator: Weak<TextureCreator<WindowContext>>) {
    //     soft_texture_default_destroy(self.tex, tex_creator);
    // }
}