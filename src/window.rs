use std::collections::HashMap;
use std::path::Path;

use sdl2::render::WindowCanvas;

use crate::components::*;
use crate::tex_man::TextureManager;

pub struct WindowBuilder {
    compos: HashMap<u32, Box<dyn BuilderCompo>>,
    tex_man: TextureManager,
}

impl WindowBuilder {
    pub fn new() -> Result<WindowBuilder, String> {
        // let (tx, rx) = mpsc::channel();
        // thread::spawn(move || {
        //     let path = Box::from(Path::new("image.bmp"));
        //     let fbt = BMPSoftTexture::new(path);
        //     tx.send(fbt).unwrap();
        // });
        // let fbt = rx.iter().next().unwrap();

        let mut compos: HashMap<u32, Box<dyn BuilderCompo>> = HashMap::new();
        compos.insert(0, Box::new(Image::from_bmp(Box::from(Path::new("image.bmp")))));

        Ok(WindowBuilder {
            compos,
            tex_man: TextureManager::new(),
        })
    }
    pub fn render(&mut self, canvas: &mut WindowCanvas) -> Result<(), Box<(dyn std::error::Error)>> {
        let tex_creator = canvas.texture_creator();
        for (_id, compo) in &mut self.compos {
            let tex = compo.render(&tex_creator, &mut self.tex_man)?;
            let guard = tex.lock().unwrap();
            canvas.copy(&guard, None, None)?;
        }
        self.tex_man.garbage_collect(tex_creator);
        Ok(())
    }
}