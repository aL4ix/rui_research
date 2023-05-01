use std::collections::btree_map::BTreeMap;
use std::error::Error;

use log::debug;
#[cfg(not(target_family = "wasm"))]
use rayon::prelude::*;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::render::WindowCanvas;

use crate::general::Geometry;
use crate::texture::TextureManager;
use crate::widgets::*;

pub struct WindowBuilder {
    widgets: BTreeMap<usize, Box<dyn Widget>>,
    geometries: BTreeMap<usize, Geometry>,
    tex_man: TextureManager,
    widget_global_id: usize,
}

impl WindowBuilder {
    pub fn new() -> Result<WindowBuilder, Box<(dyn Error)>> {
        let widgets: BTreeMap<usize, Box<dyn Widget>> = BTreeMap::new();

        Ok(WindowBuilder {
            widgets,
            geometries: Default::default(),
            tex_man: TextureManager::new(),
            widget_global_id: 1, // Starting id
        })
    }
    pub fn add_widget(&mut self, render_id: usize, mut widget: Box<dyn Widget>) {
        if widget.id() == 0 {
            while self.widgets.contains_key(&self.widget_global_id) {
                self.widget_global_id += 1;
            }
            debug!("Setting id={} automatically to {:?}", self.widget_global_id, widget);
            widget.set_id(self.widget_global_id);
            self.widget_global_id += 1;
        }
        self.widgets.insert(render_id, widget);
    }
    pub fn build_geometry(&mut self) -> Result<(), Box<(dyn Error)>> {
        // Check if new widgets are needed based on DSL
        self.geometries.clear();

        #[cfg(not(target_family = "wasm"))]
            let functional_iter = self.widgets.par_iter_mut();
        #[cfg(target_family = "wasm")]
            let functional_iter = self.widgets.iter_mut();

        self.geometries = functional_iter
            .map(|w| (*w.0, w.1.build_geometry()))
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
        let result = TextBox::get_by_id(self, 2);
        if let Ok(text) = result {
            text.set_text(&key.to_string())
        } else {
            panic!("key_down {:?}", result)
        }
    }
    pub fn event_mouse_button_down(&mut self, _mouse_btn: MouseButton, x: i32, y: i32) {
        for widget in &mut self.widgets.values_mut() {
            if widget.accepts_mouse(x, y) {
                // TODO send Window trait
                widget.event_mouse_button_down(x, y);
            }
        }
    }
}
