use std::fs;
use std::collections::btree_map::BTreeMap;
use std::error::Error;
use std::path::Path;

use glyph_brush::ab_glyph::FontArc;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;

use crate::general::{Body, Color};
use crate::tex_man::TextureManager;
use crate::widgets::*;

pub struct Window {
    widgets: BTreeMap<usize, Box<dyn Widget>>,
    bodies: BTreeMap<usize, Body>,
    tex_man: TextureManager,
}

impl Window {
    pub fn new() -> Result<Window, Box<(dyn Error)>> {
        let mut widgets: BTreeMap<usize, Box<dyn Widget>> = BTreeMap::new();

        // Multi-threaded
        // let (tx, rx) = mpsc::channel();
        // thread::spawn(move || { // Lets test if it's possible to create widgets from other threads
        //     let image = Image::from_bmp(1, Box::from(Path::new("assets/image.bmp")));
        //
        //     tx.send(image).unwrap();
        // });
        // let image = rx.iter().next().unwrap();

        // Single-threaded
        let image = Image::from_bmp(1, Box::from(Path::new("assets/image.bmp")));

        // TODO what to do with errors in widget constructors
        widgets.insert(0, Box::new(image?));

        let font_name = "assets/Nouveau_IBM.ttf".to_string();
        let file = fs::read(font_name)?;
        let font = FontArc::try_from_vec(file)?;
        let text = Text::new(2, "RUI", 300.0, font,
                             Color { r: 50, g: 50, b: 255, a: 200 });
        widgets.insert(1, Box::new(text));

        Ok(Window {
            widgets,
            bodies: Default::default(),
            tex_man: TextureManager::new(),
        })
    }
    pub fn build(&mut self) -> Result<(), Box<(dyn Error)>> {
        // TODO Check if new widgets are needed based on DSL
        self.bodies.clear();

        // Multi-threaded
        // let mut split = self.widgets.split_off(&0);
        // let (tx, rx) = mpsc::channel();
        // thread::spawn(move || {
        //     let mut bodies = BTreeMap::new();
        //     for (id, widget) in &mut split {
        //         let body = widget.build();
        //         bodies.insert(*id, body);
        //     }
        //     tx.send((bodies, split)).unwrap();
        // });
        // let (mut bodies, mut split) = rx.iter().next().unwrap();
        // self.widgets.append(&mut split);
        // self.bodies.append(&mut bodies);

        // Single-threaded
        for (id, widget) in &mut self.widgets {
            let body = widget.build();
            self.bodies.insert(*id, body);
        }

        // TODO Delete not needed widgets
        Ok(())
    }
    pub fn render(&mut self, canvas: &mut WindowCanvas) -> Result<(), Box<(dyn Error)>> {
        let tex_creator = canvas.texture_creator();
        for body in &mut self.bodies.values_mut() {
            body.render(canvas, &tex_creator, &mut self.tex_man)?;
        }

        self.tex_man.garbage_collect(tex_creator);
        Ok(())
    }
    pub fn get_widget_by_id(&mut self, id: usize) -> Option<&mut Box<dyn Widget>> {
        self.widgets.values_mut().find(|widget| widget.id() == id)
    }
    pub fn key_down(&mut self, key: Keycode) {
        if let Ok(text) = Text::get_by_id(self, 2) {
            text.set_text(&key.to_string())
        } else {
            panic!("key_down")
        }
    }
}
