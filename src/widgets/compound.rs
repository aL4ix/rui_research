use core::panic;
use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

use log::{debug, info};

use crate::general::{Geometry, Vector2D};
use crate::themes::StyleMaster;
use crate::widgets::events::{
    Event, KeyDown, KeyDownCallback, MouseButtonDown, MouseButtonDownCallback,
};
use crate::widgets::primitives::private::PrivatePrimitiveMethods;
use crate::widgets::primitives::Primitive;
use crate::widgets::{DowncastableBorrowedWidget, OwnedDynWidget, WidgetEnum, WidgetId};
use crate::window::Root;

use super::events::HasEvents;
use super::Widget;

#[derive(Debug)]
pub struct Compound {
    widgets: BTreeMap<WidgetId, OwnedDynWidget>, // wid, owned_widget
    wid: WidgetId,
    position: Vector2D<f32>,
    size: Vector2D<f32>,
    _style_master: Arc<StyleMaster>,
    event_mouse_button_down: MouseButtonDown,
    event_key_down: KeyDown,
    translated_geometry: Geometry,
    next_x: f32,
    y_size: f32,
    borrowed: HashMap<WidgetId, DowncastableBorrowedWidget>,
}

impl Compound {
    pub fn new<WENUM: WidgetEnum>(
        wid: WENUM,
        _style_master: Arc<StyleMaster>,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            widgets: Default::default(),
            wid: wid.to_wid(),
            position: Default::default(),
            size: Default::default(),
            _style_master,
            event_mouse_button_down: Default::default(),
            event_key_down: Default::default(),
            translated_geometry: Default::default(),
            next_x: Default::default(),
            y_size: Default::default(),
            borrowed: Default::default(),
        })
    }
    pub fn add_widget<T: Widget>(&mut self, widget: T) {
        let mut box_widget = Box::new(widget);
        // TODO: Factorize
        let position = self.get_next_position(&mut box_widget);
        box_widget.set_position(position);

        if box_widget.size().y() > self.y_size {
            self.y_size = box_widget.size().y();
        }
        info!("add_widget: next_x: {}", self.next_x);
        self.size = Vector2D::new(self.next_x, self.y_size);
        info!("add_widget: size: {:?}", self.size);
        self.widgets.insert(box_widget.wid(), box_widget);
    }
    pub fn widget_that_accepts_click(&mut self, x: i32, y: i32) -> WidgetId {
        for (wid, dyn_widget) in &mut self.widgets {
            if dyn_widget.will_accept_mouse_click_event(x, y) {
                return *wid;
            }
        }
        0
    }
    pub fn return_borrowed_widgets(&mut self) {
        if !self.borrowed.is_empty() {
            info!("ret_borrows len={}", self.borrowed.len());
        }

        for wid in self.borrowed.keys().cloned().collect::<Vec<_>>() {
            let down_borrow = self
                .borrowed
                .remove(&wid)
                .expect("Compound:return_borrowed_widgets remove");
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
                .expect("Compound:return_borrowed_widgets Arc::try_unwrap");
            let widget = mutex.into_inner().expect("Extract from mutex");
            debug!("{}", widget.class());
            // let rid = self
            //     .widgets
            //     .get(&wid)
            //     .expect("Compound:return_borrowed_widgets wid to rid");
            self.widgets.insert(wid, widget); // TODO Convert to Map
        }
    }
    fn get_next_position<T: Widget + ?Sized>(&mut self, widget: &mut Box<T>) -> Vector2D<f32> {
        // Factorize
        let position = (self.next_x, 0.0).into();
        self.next_x += widget.size().x();
        position
    }
}

impl Primitive for Compound {
    fn class_name() -> &'static str {
        stringify!(Compound)
    }
    // TODO change to macro
    fn class(&self) -> &'static str {
        Self::class_name()
    }
    fn wid(&self) -> usize {
        self.wid
    }
    fn set_wid(&mut self, nid: usize) {
        self.wid = nid
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
        self.position = position;
        /*
        Set position for each widget.

        Maybe Compound should be called Component.
        Then now what about sizers?
        It would make sense to have sizer abilities into Component, but should we need to have it at WindowBuilder?
        Maybe no, because window can have special abilities like menus, status bars, toolbars, docking, so if after that
        you want to have sizer abilities then use a Component, like in HTML, you declare a main and then start from there.

        How can we implement a RetroRenderer?
        The logic says use a Component, and then what? we don't need sizer, unless we need, because the "Primitive" needs
        to be sized to the size of the window. It would make sense to have everything in window have the full size, but
        still it sounds like we need some kind of sizing in window is it?
        If Component is full sized then Component can full size the "Primitive", which should be a Bitmap right?, just
        we need to have the ability to create it with some raw data.
        Or maybe this should be Image, Components are made up from Widgets. You cannot declare new Widgets, is it?
        I think it was supposed to be that way..
        Component was about like tabs sheets, textbox with label.
        Yeah instead of having divs of divs have Components.
        What about new things? are they Widgets? what could be new Widgets? RadioButton, CheckBox, Calendar, DateSelector,
        ComboBox, Progress, Spinner,
        Component: Card, Table, List group, Modal, Popover, Toast, Tooltip.
        Or maybe special things like Menubar.
        Maybe the distinction comes from the functionality, like Calendar could be implemented as a group of buttons but
        it needs funcionality like get_date().

        Window sizer?
        It is definetely a special kind of spacer, because some components wont be affected, like bars, but the rest is
        affected, so i guess it would make sense to have a Component and force the user to have one and only one.
        Then from there the user can decide their own setting.

        How to implement a carousel?

        For a "screen reader", what about a DateSelector?
        Simple, read the selected date. What about setting it? speak and set the date.
        How can you read a calendar?
        Go to march, what is the last day? Friday.
        So we do need special abilities for Calendar.
        Almost everything that is on top, will go to a special place in window, like menubars, status, toolbars, docks,
        and the screen reader can interpret those in a special way. Same for calendar, it can announce there is a calendar
        open.

         */
        self.set_needs_translation(true);
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
        self.next_x = 0.0;
        let mut geometries = Vec::with_capacity(self.widgets.len());
        for dyn_widget in self.widgets.values_mut() {
            info!("translate_geometry: {}", dyn_widget.class());
            // let geometry = if dyn_widget.needs_translation() {
            // Refactorize
            let position = (self.next_x, 0.0).into();
            let dyn_widget_size = dyn_widget.size();
            self.next_x += dyn_widget_size.x();
            if dyn_widget_size.y() > self.y_size {
                self.y_size = dyn_widget_size.y();
            }
            self.y_size = dyn_widget_size.y();
            dyn_widget.set_position(position);

            let geometry = dyn_widget.translate_geometry();
            // } else {
            //     dyn_widget.clone_translated_geometry()
            // };
            geometries.push(geometry);
        }
        self.size = Vector2D::new(self.next_x, self.y_size);
        self.translated_geometry = Geometry::new_from_geometries(Self::class_name(), geometries);
        self.translated_geometry.clone()
    }

    fn build_geometry(&mut self) -> Geometry {
        if self.needs_update() {
            info!("build_geometry: needs_update");
            self.update_geometry();
            self.set_needs_update(false);
            self.translate_geometry()
        } else if self.needs_translation() {
            return self.translate_geometry();
        } else {
            return self.clone_translated_geometry();
        }
    }
}

impl PrivatePrimitiveMethods for Compound {
    fn update_geometry(&mut self) {
        for dyn_widget in self.widgets.values_mut() {
            if dyn_widget.needs_update() {
                info!("update_geometry: {}", dyn_widget.class());
                dyn_widget.update_geometry();
            }
        }
    }
    fn needs_update(&self) -> bool {
        for dyn_widget in self.widgets.values() {
            if dyn_widget.needs_update() {
                return true;
            }
        }
        false
    }
    fn set_needs_update(&mut self, needs_update: bool) {
        for dyn_widget in self.widgets.values_mut() {
            dyn_widget.set_needs_update(needs_update);
        }
    }
    fn needs_translation(&self) -> bool {
        for dyn_widget in self.widgets.values() {
            if dyn_widget.needs_translation() {
                return true;
            }
        }
        false
    }
    fn set_needs_translation(&mut self, needs_translation: bool) {
        // if needs_translation {
        //     panic!("set_needs_translation This should not have been called");
        // }
        for dyn_widget in self.widgets.values_mut() {
            dyn_widget.set_needs_translation(needs_translation);
        }
    }
    fn clone_geometry(&self) -> Geometry {
        let mut geometries = Vec::with_capacity(self.widgets.len());
        for dyn_widget in self.widgets.values() {
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

impl HasEvents for Compound {
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

impl Widget for Compound {}

impl Root for Compound {
    fn get_down_widget_by_id(
        &mut self,
        wid: super::WidgetId,
    ) -> Option<super::DowncastableBorrowedWidget> {
        if let Some(borrowed) = self.borrowed.get(&wid) {
            info!(
                "down_borrow strong_count={}",
                borrowed.get_borrowed_strong_count()
            );
            return Some(borrowed.clone());
        }

        info!(
            "Compound get_down_widget_by_id widgets.keys {:?}",
            self.widgets.keys()
        );
        let opt_widget = self.widgets.remove(&wid);
        let widget = opt_widget?;
        let class = widget.class();
        let type_id = widget.type_id();
        let dyn_widget = Arc::new(Mutex::new(widget));
        let dowcastable = DowncastableBorrowedWidget::new(type_id, dyn_widget, class);
        self.borrowed.insert(wid, dowcastable.clone());
        Some(dowcastable)
    }

    fn children(&self) -> &BTreeMap<WidgetId, OwnedDynWidget> {
        &self.widgets
    }
}
