use std::collections::btree_map::BTreeMap;
use std::collections::HashMap;
use std::error::Error;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

use log::{debug, info};
#[cfg(not(target_family = "wasm"))]
use rayon::prelude::*;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::render::WindowCanvas;

use crate::general::Geometry;
use crate::texture::TextureManager;
use crate::utils::Downcast;
use crate::widgets::primitives::Primitive;
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
    wid_and_cwid: HashMap<WidgetId, WidgetId>, // wid, cwid
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
            wid_and_cwid: Default::default(),
        })
    }
    pub fn add_widget<W: Widget>(&mut self, render_id: isize, widget: W) {
        // Compound
        let opt_custom_widget = widget.downcast_ref::<Compound>();
        if let Some(custom_widget) = opt_custom_widget {
            for child in custom_widget.children().values() {
                self.wid_and_cwid.insert(child.wid(), custom_widget.wid());
            }
        }
        // End Compound
        self.wid_and_rid.insert(widget.wid(), render_id);
        self.widgets.insert(render_id, Box::new(widget));
    }
    pub fn build_geometry(&mut self) -> Result<(), Box<(dyn Error)>> {
        // Check if new widgets are needed based on DSL
        // TODO, make in parallel
        self.return_borrowed_widgets();
        for cwid in self.wid_and_cwid.values() {
            let crid = self
                .wid_and_rid
                .get(cwid)
                .ok_or("build_geometry cwid not in wid_and_rid")?;
            let widget = self
                .widgets
                .get_mut(crid)
                .ok_or("build_geometry expected crid in widgets")?;
            let cwidget = (**widget)
                .downcast_mut::<Compound>()
                .ok_or("Could not downcast Compound")?;
            cwidget.return_borrowed_widgets();
        }

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
                    .get(rid)
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
        debug!("event_mouse_button_down Clicked");
        let it = self.widgets.iter_mut().rev();
        // TODO: Cannot use find(), why?
        let mut found = None;
        for (rid, widget) in it {
            if widget.will_accept_mouse_click_event(x, y) {
                found = Some((rid, widget));
                debug!("Found rid:{}", rid);
                break;
            }
        }

        if let Some((rid, widget)) = found {
            self.focused_wid = self
                .wid_and_rid
                .iter()
                .find(|(_, internal_rid)| *internal_rid == rid)
                .map(|(wid, _)| *wid);
            debug!(
                "event_mouse_button_down Focused_wid: {:?}",
                self.focused_wid
            );
            let option_compound = (widget.as_mut() as &mut dyn Widget).downcast_mut::<Compound>();
            if let Some(compound) = option_compound {
                let wcid = compound.widget_that_accepts_click(x, y);
                for (tcwid, child) in compound.children() {
                    if *tcwid == wcid {
                        let event_callback = child.event_mouse_button_down();
                        info!("Clicked component wid:{}", wcid);
                        (event_callback.deref())(self, x, y);
                        break;
                    }
                }
            } else {
                let event_callback = widget.event_mouse_button_down();
                (event_callback.deref())(self, x, y);
            }
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

        if let Some(rid) = self.wid_and_rid.get(&wid) {
            let opt_widget = self.widgets.remove(rid);
            let widget = opt_widget?;
            let _class = widget.class();
            let type_id = widget.type_id();
            info!("wid_down_borrow type_id: {:?}", type_id);
            let dyn_widget = Arc::new(Mutex::new(widget));
            let downcastable = DowncastableBorrowedWidget::new(type_id, dyn_widget, _class);
            self.borrowed.insert(wid, downcastable.clone());
            return Some(downcastable);
        }

        info!("down_borrow widget.keys {:?}", self.widgets.keys());
        info!("down_borrow wid_container {:?}", self.wid_and_cwid);
        if let Some(cwid) = self.wid_and_cwid.get(&wid) {
            let crid = self.wid_and_rid.get(cwid)?;
            let cwidget = self.widgets.get_mut(crid)?;
            let class = cwidget.class();
            let container = (**cwidget)
                .downcast_mut::<Compound>()
                .unwrap_or_else(|| panic!("Wanted Compound found {}", class));
            return container.get_down_widget_by_id(wid);
        }

        None
    }
    fn return_borrowed_widgets(&mut self) {
        if !self.borrowed.is_empty() {
            info!("return_borrowed_widgets len={}", self.borrowed.len());
        }

        for wid in self.borrowed.keys().cloned().collect::<Vec<_>>() {
            let down_borrow = self
                .borrowed
                .remove(&wid)
                .expect("window_builder:WindowBuilder:return_borrowed_widgets remove");
            let dyn_widget = down_borrow.own_dyn_widget();
            let count = Arc::strong_count(&dyn_widget);
            info!("wid={} count={}", wid, count);
            if count > 1 {
                panic!(
                    "Expected wid={} to have strong_count of 1, found {}. 
                    Note to the dev, don't leave dangling pointers after leaving an event.",
                    wid, count
                );
            }
            let mutex = Arc::try_unwrap(dyn_widget)
                .expect("window_builder:WindowBuilder:return_borrowed_widgets Arc::try_unwrap");
            let widget = mutex.into_inner().expect("Extract from mutex");
            debug!("{}", widget.class());
            let rid = self
                .wid_and_rid
                .get(&wid)
                .expect("window_builder:WindowBuilder:return_borrowed_widgets wid to rid");
            self.widgets.insert(*rid, widget);
        }
    }
}

impl Root for WindowBuilder {
    fn get_down_widget_by_id(&mut self, wid: WidgetId) -> Option<DowncastableBorrowedWidget> {
        self.wid_down_borrow(wid)
    }

    fn children(&self) -> &BTreeMap<WidgetId, OwnedDynWidget> {
        todo!()
    }
}
