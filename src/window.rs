use std::{fs, thread};
use std::collections::btree_map::BTreeMap;
// use std::collections::BTreeMap;
use std::error::Error;
use std::path::Path;
use std::sync::mpsc;

use glyph_brush::ab_glyph::FontArc;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;

use crate::general::Color;
use crate::tex_man::TextureManager;
use crate::widgets::*;

pub struct WindowBuilder {
    widgets: BTreeMap<usize, Box<dyn Widget>>,
    tex_man: TextureManager,
}

impl WindowBuilder {
    pub fn new() -> Result<WindowBuilder, Box<(dyn Error)>> {
        let mut widgets: BTreeMap<usize, Box<dyn Widget>> = BTreeMap::new();

        let (tx, rx) = mpsc::channel();
        thread::spawn(move || { // Lets test if it's possible to create widgets from other threads
            let image = Image::from_bmp(1, Box::from(Path::new("image.bmp")));

            tx.send(image).unwrap();
        });
        let image = rx.iter().next().unwrap();
        widgets.insert(0, Box::new(image?));

        let font_name = "Nouveau_IBM.ttf".to_string();
        let file = fs::read(font_name.clone())?;
        let font = FontArc::try_from_vec(file)?;
        let text = Text::new(2, "RUI", 300.0, font,
                             Color { r: 50, g: 50, b: 255, a: 200 });
        widgets.insert(1, Box::new(text));

        Ok(WindowBuilder {
            widgets,
            tex_man: TextureManager::new(),
        })
    }
    pub fn render(&mut self, canvas: &mut WindowCanvas) -> Result<(), Box<(dyn Error)>> {
        let tex_creator = canvas.texture_creator();
        for (_id, widget) in &mut self.widgets {
            let mut body = widget.build()?;
            body.render(canvas, &tex_creator, &mut self.tex_man)?;
        }

        self.tex_man.garbage_collect(tex_creator);
        Ok(())
    }
    pub fn get_widget_by_id(&mut self, id: usize) -> Option<&mut Box<dyn Widget>> {
        for (_, widget) in &mut self.widgets {
            if widget.id() == id {
                return Some(widget);
            }
        }
        None
    }
    pub fn key_down(&mut self, key: Keycode) {
        let text = Text::get_by_id(self, 2).expect("key_down");
        text.set_text(&key.to_string())
    }
}
