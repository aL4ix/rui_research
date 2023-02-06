use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

use sdl2::render::WindowCanvas;

use crate::components::*;

pub struct WindowBuilder {
    compos: HashMap<u32, Box<dyn BuilderCompo>>
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
            compos
        })
    }
    pub fn render(&mut self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let tex_creator = Rc::new(canvas.texture_creator());
        let context = RenderContext::new(Rc::downgrade(&tex_creator));
        for (id, compo) in &mut self.compos {
            let tex = compo.render(context.clone()).map_err(|e| e.to_string())?;
            let guard = tex.lock().unwrap();
            canvas.copy(&guard, None, None)?;
        }
        Ok(())
    }
}