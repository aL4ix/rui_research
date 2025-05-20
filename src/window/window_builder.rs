use std::collections::btree_map::BTreeMap;
use std::error::Error;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

use log::debug;
#[cfg(not(target_family = "wasm"))]
use rayon::prelude::*;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::render::WindowCanvas;

use crate::general::Geometry;
use crate::texture::TextureManager;
use crate::widgets::*;
use crate::window::Root;

pub struct WindowBuilder {
    rid_and_wid: BTreeMap<usize, usize>,
    widget_man: WidgetManager,
    geometries: BTreeMap<usize, Geometry>,
    tex_man: TextureManager,
    width: u32,
    height: u32,
    borrowed: BTreeMap<usize, DynWidget>,
}

impl WindowBuilder {
    pub fn new() -> Result<WindowBuilder, Box<(dyn Error)>> {
        Ok(WindowBuilder {
            rid_and_wid: Default::default(),
            widget_man: Default::default(),
            geometries: Default::default(),
            tex_man: TextureManager::new(),
            width: 1024,
            height: 768,
            borrowed: Default::default()
        })
    }
    pub fn add_widget<T: Widget>(&mut self, render_id: usize, widget: T, wid: usize) {
        let dw: DynWidget = Arc::new(Mutex::new(Box::new(widget)));
        self.widget_man.insert(wid, dw);
        self.rid_and_wid.insert(render_id, wid);
    }
    fn get_widgets_to_render(&self) -> BTreeMap<usize, DynWidget> {
        let widgets_to_render: BTreeMap<usize, DynWidget> = self.rid_and_wid.iter()
        .map(|(rid, wid)| (*rid, self.widget_man.get(*wid).expect("FAILED: get_widgets_to_render()").dyn_wid())).collect();
        return widgets_to_render;
    }
    pub fn build_geometry(&mut self) -> Result<(), Box<(dyn Error)>> {
        // Check if new widgets are needed based on DSL
        self.geometries.clear();

        let mut binding = self.get_widgets_to_render();
        #[cfg(not(target_family = "wasm"))]
        // let functional_iter = binding.par_iter_mut();
        let functional_iter = binding.iter_mut();
        #[cfg(target_family = "wasm")]
        let functional_iter = binding.iter_mut();

        self.geometries = functional_iter
            .map(|w| (*w.0, w.1.lock().expect("window_builder:WindowBuilder:build_geometry").build_geometry()))
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
    pub fn event_key_down(&mut self, key: Keycode) {
        let result = TextBox::get_by_id(self, 2);
        if let Ok(text) = result {
            text.lock().expect("window_builder:WindowBuilder:event_key_down").set_text(&key.to_string())
        } else {
            debug!("key_down {:?}", result)
        }
    }
    pub fn event_mouse_button_down(&mut self, _mouse_btn: MouseButton, x: i32, y: i32) {
        let mut binding = self
            .get_widgets_to_render();
        let found = binding
            .iter_mut()
            .rev()
            .find(|(_, w)| w.lock().expect("window_builder:event_mouse_button_down").accepts_mouse(x, y));
        if let Some((_, widget)) = found {
            let event_callback = widget.lock().expect("window_builder:event_mouse_button_down").event_mouse_button_down();
            (event_callback.deref())(self, x, y);
        }
        drop(binding);
        // self.re_own_widgets();
    }
    fn re_own_widgets(&mut self) {
        for (rid, wid) in self.rid_and_wid.clone() {
            let widget = self.widget_man.remove(wid);
            self.rid_and_wid.remove(&rid);
            let count = Arc::strong_count(&widget);
            println!("{}", count);
            if count > 1 {
                continue;
            }
            let a = Arc::try_unwrap(widget).expect("msg1");
            let b = a.into_inner().expect("msg2");
            println!("{}" ,b.class());
        }
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
}

impl Root for WindowBuilder {
    fn get_widget_by_id_dyn(&mut self, wid: usize) -> Option<DynWidget> {
        self.widget_man.get(wid).map(|f| f.dyn_wid())
    }
    
    fn get_down_widget_by_id(&mut self, wid: usize) -> Option<DowncastableWidget> {
        self.widget_man.get(wid)
    }
}
