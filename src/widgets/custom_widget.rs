use core::panic;
use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

use log::{debug, info};

use crate::general::{Geometry, Vector2D};
use crate::widgets::events::{
    Event, KeyDown, KeyDownCallback, MouseButtonDown, MouseButtonDownCallback,
};
use crate::widgets::primitives::private::PrivatePrimitiveMethods;
use crate::widgets::primitives::Primitive;
use crate::widgets::themes::StyleMaster;
use crate::widgets::{DowncastableBorrowedWidget, OwnedDynWidget, WidgetId};
use crate::window::Root;

use super::events::HasEvents;
use super::Widget;

#[derive(Debug)]
pub struct CustomWidget {
    widgets: BTreeMap<WidgetId, OwnedDynWidget>, // wid, owned_widget
    nid: usize,
    position: Vector2D<f32>,
    size: Vector2D<f32>,
    _style_master: Arc<StyleMaster>,
    event_mouse_button_down: MouseButtonDown,
    event_key_down: KeyDown,
    translated_geometry: Geometry,
    next_x: f32,
    next_y: f32,
    borrowed: HashMap<WidgetId, DowncastableBorrowedWidget>,
}

impl CustomWidget {
    pub fn new(nid: usize, _style_master: Arc<StyleMaster>) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            widgets: Default::default(),
            nid,
            position: Default::default(),
            size: Default::default(),
            _style_master,
            event_mouse_button_down: Default::default(),
            event_key_down: Default::default(),
            translated_geometry: Default::default(),
            next_x: Default::default(),
            next_y: Default::default(),
            borrowed: Default::default(),
        })
    }
    pub fn add_widget<T: Widget>(&mut self, mut widget: T) {
        widget.set_position((self.next_x, 0.0).into());
        self.next_x += widget.size().x();
        if widget.size().y() > self.next_y {
            self.next_y = widget.size().y();
        }
        info!("add_widget: next_x: {}", self.next_x);
        self.size = Vector2D::new(self.next_x, self.next_y);
        info!("add_widget: size: {:?}", self.size);
        self.widgets.insert(widget.nid(), Box::new(widget));
    }
    pub fn widget_that_accepts_click(&mut self, x: i32, y: i32) -> WidgetId {
        for (wid, dyn_widget) in &mut self.widgets {
            if dyn_widget.will_accept_mouse_click_event(x, y) {
                return *wid
            }
        }
        return 0
    }
    pub fn return_borrowed_widgets(&mut self) {
        if !self.borrowed.is_empty() {
            info!("ret_borrows len={}", self.borrowed.len());
        }

        for wid in self.borrowed.keys().cloned().collect::<Vec<_>>() {
            let down_borrow = self
                .borrowed
                .remove(&wid)
                .expect("CustomWidget:return_borrowed_widgets remove");
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
                .expect("CustomWidget:return_borrowed_widgets Arc::try_unwrap");
            let widget = mutex.into_inner().expect("Extract from mutex");
            debug!("{}", widget.class());
            // let rid = self
            //     .widgets
            //     .get(&wid)
            //     .expect("CustomWidget:return_borrowed_widgets wid to rid");
            self.widgets.insert(wid, widget); // TODO Convert to Map
        }
    }
}

impl Primitive for CustomWidget {
    fn class_name() -> &'static str {
        "CustomWidget"
    }
    // TODO change to macro
    fn class(&self) -> &'static str {
        Self::class_name()
    }
    fn nid(&self) -> usize {
        self.nid
    }
    fn set_nid(&mut self, nid: usize) {
        self.nid = nid
    }
    fn x(&self) -> f32 {
        self.position.x()
    }
    fn y(&self) -> f32 {
        self.position.y()
    }
    fn position(&self) -> &Vector2D<f32> {
        &self.position
    }
    fn set_position(&mut self, position: Vector2D<f32>) {
        self.position = position
    }
    fn width(&self) -> f32 {
        self.size.x()
    }
    fn height(&self) -> f32 {
        self.size.y()
    }
    fn size(&mut self) -> &Vector2D<f32> {
        &self.size
    }
    fn translate_geometry(&mut self) -> Geometry {
        // if self.needs_translation() == false {
        //     return self.translated_geometry.clone();
        // }
        let mut geometries = Vec::with_capacity(self.widgets.len());
        for (_, dyn_widget) in &mut self.widgets {
            info!("translate_geometry: {}", dyn_widget.class());
            // let geometry = if dyn_widget.needs_translation() {
                let geometry = dyn_widget.translate_geometry();
            // } else {
            //     dyn_widget.clone_translated_geometry()
            // };
            geometries.push(geometry);
        }
        self.translated_geometry = Geometry::new_from_geometries(Self::class_name(), geometries);
        self.translated_geometry.clone()
    }
    
    fn build_geometry(&mut self) -> Geometry {
        if self.needs_update() {
            info!("build_geometry");
            self.update_geometry();
            self.set_needs_update(false);
            return self.translate_geometry();
        } else if self.needs_translation() {
            return self.translate_geometry();
        } else {
            return self.clone_translated_geometry();
        }
    }
}

impl PrivatePrimitiveMethods for CustomWidget {
    fn update_geometry(&mut self) {
        for (_, dyn_widget) in &mut self.widgets {
            if dyn_widget.needs_update() {
                info!("update_geometry: {}", dyn_widget.class());
                dyn_widget.update_geometry();
            }
        }
    }
    fn needs_update(&self) -> bool {
        for (_, dyn_widget) in &self.widgets {
            if dyn_widget.needs_update() {
                return true;
            }
        }
        false
    }
    fn set_needs_update(&mut self, needs_update: bool) {
        for (_, dyn_widget) in &mut self.widgets {
            dyn_widget.set_needs_update(needs_update);
        }
    }
    fn needs_translation(&self) -> bool {
        for (_, dyn_widget) in &self.widgets {
            if dyn_widget.needs_translation() {
                return true;
            }
        }
        false
    }
    fn set_needs_translation(&mut self, needs_translation: bool) {
        if needs_translation {
            panic!("set_needs_translation This should not have been called");
        }
        for (_, dyn_widget) in &mut self.widgets {
            dyn_widget.set_needs_translation(needs_translation);
        }
    }
    fn clone_geometry(&self) -> Geometry {
        let mut geometries = Vec::with_capacity(self.widgets.len());
        for (_, dyn_widget) in &self.widgets {
            geometries.push(dyn_widget.clone_geometry());
        }
        Geometry::new_from_geometries(Self::class_name(), geometries)
    }

    fn set_translated_geometry(&mut self, _translated_geometry: Geometry) {
        panic!("set_translated_geometry This should have not been called!");
    }
    fn clone_translated_geometry(&self) -> Geometry {
        self.translated_geometry.clone()
    }
}

impl HasEvents for CustomWidget {
    fn event_mouse_button_down(&self) -> Arc<MouseButtonDownCallback> {
        self.event_mouse_button_down.clone_callback()
    }
    fn set_event_mouse_button_down(&mut self, callback: MouseButtonDownCallback) {
        self.event_mouse_button_down = MouseButtonDown {
            callback: Arc::new(callback),
        }
    }
    fn event_key_down(&self) -> Arc<KeyDownCallback> {
        self.event_key_down.clone_callback()
    }
    fn set_event_key_down(&mut self, callback: KeyDownCallback) {
        self.event_key_down = KeyDown {
            callback: Arc::new(callback),
        }
    }
}

impl Widget for CustomWidget {}

impl Root for CustomWidget {
    fn get_down_widget_by_id(
        &mut self,
        wid: super::WidgetId,
    ) -> Option<super::DowncastableBorrowedWidget> {
        if let Some(borrowed) = self.borrowed.get(&wid) {
            info!("down_borrow strong_count={}", borrowed.get_borrowed_strong_count());
            return Some(borrowed.clone());
        }

            info!("CustomWidget get_down_widget_by_id widgets.keys {:?}", self.widgets.keys());
            let opt_widget = self.widgets.remove(&wid);
            let widget = opt_widget?;
            let class = widget.class();
            let type_id = widget.type_id();
            let dyn_widget = Arc::new(Mutex::new(widget));
            let dowcastable = DowncastableBorrowedWidget::new(type_id, dyn_widget, class);
            self.borrowed.insert(wid, dowcastable.clone());
            return Some(dowcastable)

    }

    fn children(&self) -> &BTreeMap<WidgetId, OwnedDynWidget> {
        &self.widgets
    }
}
