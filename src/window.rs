use std::{fs, thread};
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::mpsc;

use glyph_brush::ab_glyph::FontArc;
use sdl2::render::WindowCanvas;

use crate::components::*;
use crate::general::Color;
use crate::tex_man::TextureManager;

pub struct WindowBuilder {
    compos: BTreeMap<u32, Box<dyn BuilderCompo>>,
    tex_man: TextureManager,
}

impl WindowBuilder {
    pub fn new() -> Result<WindowBuilder, Box<(dyn std::error::Error)>> {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let compo = Box::new(Image::from_bmp(Box::from(Path::new("image.bmp"))));
            tx.send(compo).unwrap();
        });
        let image = rx.iter().next().unwrap();

        let mut compos: BTreeMap<u32, Box<dyn BuilderCompo>> = BTreeMap::new();
        compos.insert(0, image);

        let font_name = "Nouveau_IBM.ttf".to_string();
        let file = fs::read(font_name.clone())?;
        let font = FontArc::try_from_vec(file)?;
        let text = Text::new("RUI", 300.0, font,
                             Color { r: 50, g: 50, b: 255, a: 128 });
        compos.insert(1, Box::new(text));

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