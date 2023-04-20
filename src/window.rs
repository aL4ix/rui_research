use std::collections::btree_map::BTreeMap;
use std::error::Error;

use log::info;
#[cfg(not(target_family = "wasm"))]
use rayon::prelude::*;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::render::WindowCanvas;

use crate::general::Geometry;
use crate::texture::TextureManager;
use crate::widgets::*;

pub struct WindowSpecs {
    widgets: BTreeMap<usize, Box<dyn Widget>>,
    geometries: BTreeMap<usize, Geometry>,
    tex_man: TextureManager,
}


impl WindowSpecs {
    pub fn new() -> Result<WindowSpecs, Box<(dyn Error)>> {
        let widgets: BTreeMap<usize, Box<dyn Widget>> = BTreeMap::new();

        Ok(WindowSpecs {
            widgets,
            geometries: Default::default(),
            tex_man: TextureManager::new(),
        })
    }
    pub fn add_widget(&mut self, render_id: usize, widget: Box<dyn Widget>) {
        self.widgets.insert(render_id, widget);
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
    pub fn event_key_down(&mut self, key: Keycode) {
        if let Ok(text) = Text::get_by_id(self, 2) {
            text.set_text(&key.to_string())
        } else {
            panic!("key_down")
        }
    }
    pub fn event_mouse_button_down(&self, mouse_btn: MouseButton, x: i32, y: i32) {
        info!("{:?} {} {}", mouse_btn, x, y);
    }
}

pub trait Window {
    fn get_specs(&self) -> &WindowSpecs;
    fn event_key_down(&mut self, key: Keycode);
    fn event_mouse_button_down(&self, mouse_btn: MouseButton, x: i32, y: i32);
    fn build(&mut self) -> Result<(), Box<(dyn Error)>>;
    fn render(&mut self) -> Result<(), Box<(dyn Error)>>;
    fn clear_canvas(&mut self);
    fn present_canvas(&mut self);
}