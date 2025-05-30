use std::cell::RefCell;
use std::collections::btree_map::BTreeMap;
use std::collections::HashMap;
use std::error::Error;
use std::ops::Deref;
use std::rc::Rc;

use log::{debug, info};
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
    wid_and_rid: BTreeMap<WidgetId, isize>,
    widgets: BTreeMap<isize, OwnedDynWidget>, // rid, owned_widget
    geometries: BTreeMap<isize, Geometry>,    // rid, geometry
    tex_man: TextureManager,
    width: u32,
    height: u32,
    borrowed: HashMap<WidgetId, DowncastableBorrowedWidget>,
    focused_wid: Option<WidgetId>,
}

impl WindowBuilder {
    pub fn new() -> Result<WindowBuilder, Box<(dyn Error)>> {
        Ok(WindowBuilder {
            wid_and_rid: Default::default(),
            widgets: Default::default(),
            geometries: Default::default(),
            tex_man: TextureManager::new(),
            width: 1024,
            height: 768,
            borrowed: Default::default(),
            focused_wid: None,
        })
    }
    pub fn add_widget<W: Widget, WENUM: WidgetEnum>(
        &mut self,
        render_id: isize,
        widget: W,
        wenum: WENUM,
    ) {
        self.widgets.insert(render_id, Box::new(widget));
        self.wid_and_rid.insert(wenum.to_wid(), render_id);
    }
    pub fn build_geometry(&mut self) -> Result<(), Box<(dyn Error)>> {
        // Check if new widgets are needed based on DSL
        self.wid_ret_borrows();
        self.geometries.clear();

        let binding = &mut self.widgets;
        #[cfg(not(target_family = "wasm"))]
        let functional_iter = binding.par_iter_mut();
        #[cfg(target_family = "wasm")]
        let functional_iter = binding.iter_mut();

        self.geometries = functional_iter
            .map(|(rid, widget)| (*rid, widget.build_geometry()))
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
        debug!("event_key_down({:?})", key);
        if let Some(wid) = self.focused_wid {
            if let Some(rid) = self.wid_and_rid.get(&wid) {
                let widget = self
                    .widgets
                    .get(&rid)
                    .expect("window_builder:WindowBuilder:event_key_down");
                let event_callback = widget.event_key_down();
                (event_callback.deref())(self, key);
            } else {
                self.focused_wid = None;
            }
        } else {
            debug!("event_key_down None")
        }
    }
    pub fn event_mouse_button_down(&mut self, _mouse_btn: MouseButton, x: i32, y: i32) {
        let it = self.widgets.iter_mut().rev();
        // TODO: Cannot use find(), why?
        let mut found = None;
        for (rid, widget) in it {
            if widget.will_accept_mouse_click_event(x, y) {
                found = Some((rid, widget));
                break;
            }
        }

        if let Some((rid, widget)) = found {
            self.focused_wid = self
                .wid_and_rid
                .iter()
                .find(|(_, internal_rid)| *internal_rid == rid)
                .map(|(wid, _)| *wid);
            debug!("{:?}", self.focused_wid);
            let event_callback = widget.event_mouse_button_down();
            (event_callback.deref())(self, x, y);
        }
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    fn wid_down_borrow(&mut self, wid: WidgetId) -> Option<DowncastableBorrowedWidget> {
        info!("down_borrow wid={}", wid);

        if let Some(borrowed) = self.borrowed.get(&wid) {
            info!(
                "down_borrow strong_count={}",
                borrowed.get_borrowed_strong_count()
            );
            return Some(borrowed.clone());
        }

        let rid = self.wid_and_rid.get(&wid)?;
        let opt_widget = self.widgets.remove(&rid);
        if let Some(widget) = opt_widget {
            let type_id = widget.type_id();
            let dyn_widget = Rc::new(RefCell::new(widget));
            let borrowed = DowncastableBorrowedWidget::new(type_id, dyn_widget);
            self.borrowed.insert(wid, borrowed.clone());
            return Some(borrowed);
        }
        info!("down_borrow {:?}", self.widgets.keys());
        None
    }
    fn wid_ret_borrows(&mut self) {
        if self.borrowed.len() > 0 {
            info!("ret_borrows len={}", self.borrowed.len());
        }

        for wid in self.borrowed.keys().cloned().collect::<Vec<_>>() {
            let down_borrow = self
                .borrowed
                .remove(&wid)
                .expect("window_builder:WindowBuilder:ret_borrows remove");
            let dyn_widget = down_borrow.own_dyn_widget();
            let count = Rc::strong_count(&dyn_widget);
            info!("wid={} count={}", wid, count);
            if count > 1 {
                panic!(
                    "Expected wid={} to have strong_count of 1, found {}",
                    wid, count
                );
            }
            let mutex = Rc::try_unwrap(dyn_widget)
                .expect("window_builder:WindowBuilder:ret_borrows Arc::try_unwrap");
            let widget = mutex.into_inner();
            debug!("{}", widget.class());
            let rid = self
                .wid_and_rid
                .get(&wid)
                .expect("window_builder:WindowBuilder:ret_borrows wid to rid");
            self.widgets.insert(*rid, widget);
        }
    }
}

impl Root for WindowBuilder {
    fn get_down_widget_by_id(&mut self, wid: WidgetId) -> Option<DowncastableBorrowedWidget> {
        self.wid_down_borrow(wid)
    }
}
