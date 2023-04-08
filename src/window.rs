use std::collections::btree_map::BTreeMap;
use std::error::Error;
use std::path::Path;

use glyph_brush::ab_glyph::FontArc;
#[cfg(not(target_family = "wasm"))]
use rayon::prelude::*;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;

use crate::general::{Color, Geometry, Vector2D};
use crate::tex_man::TextureManager;
use crate::utils::Assets;
use crate::widgets::*;

pub struct Window {
    widgets: BTreeMap<usize, Box<dyn Widget>>,
    geometries: BTreeMap<usize, Geometry>,
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

        let font_path = "assets/Nouveau_IBM.ttf";
        let font_vec = Assets::read(font_path)?;
        let font = FontArc::try_from_vec(font_vec)?;
        let text = Text::new(2, "RUI", 300.0, font,
                             Color::new(50, 50, 255, 200));
        widgets.insert(2, Box::new(text));

        let shape = Shape::square(Vector2D::new(100.0, 50.0), 0,
                                  Color::new(255, 255, 255, 255));
        widgets.insert(1, Box::from(shape));

        Ok(Window {
            widgets,
            geometries: Default::default(),
            tex_man: TextureManager::new(),
        })
    }
    pub fn build(&mut self) -> Result<(), Box<(dyn Error)>> {
        // Check if new widgets are needed based on DSL
        self.geometries.clear();

        #[cfg(not(target_family = "wasm"))]
            let functional_iter = self.widgets.par_iter_mut();
        #[cfg(target_family = "wasm")]
            let functional_iter = self.widgets.iter_mut();

        self.geometries = functional_iter
            .map(|w| (*w.0, w.1.build()))
            .collect();

        // Delete not needed widgets
        Ok(())
    }
    pub fn render(&mut self, canvas: &mut WindowCanvas) -> Result<(), Box<(dyn Error)>> {
        let tex_creator = canvas.texture_creator();
        for geometry in &mut self.geometries.values_mut() {
            geometry.render(canvas, &tex_creator, &mut self.tex_man)?;
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
