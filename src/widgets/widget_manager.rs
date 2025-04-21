
use std::any::TypeId;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use super::Widget;

pub type DynWidget = Arc<Mutex<dyn Widget + 'static>>;
pub type WidgetT<T> = Arc<Mutex<T>>;

pub trait WidgetEnum: Clone + Copy {
    fn to_isize(self) -> isize;
}

#[derive(Clone, Debug)]
pub struct DowncastableWidget {
    type_id: TypeId,
    dyn_widget: DynWidget
}

impl DowncastableWidget {
    pub fn dyn_wid(&self) -> DynWidget {
        self.dyn_widget.clone()
    }
    pub fn wid_t<T: Widget>(&self) -> Option<WidgetT<T>> {
        let ti = TypeId::of::<T>();
        if ti == self.type_id {
            let r = Self::downcast_dyn_widget(self.dyn_widget.clone());
            return Some(r);
        }
        None
    }
    fn downcast_dyn_widget<T: Widget>(widget: DynWidget) -> WidgetT<T> where Self: Sized {
        let new_arc = unsafe { Arc::from_raw(Arc::into_raw(widget) as *const Mutex<T>) };
        new_arc
    }
}

#[derive(Default)]
pub struct WidgetManager {
    widgets: BTreeMap<usize, DowncastableWidget>,
}

impl WidgetManager {
    pub fn insert(&mut self, wid: usize, widget: DynWidget) {
        let dw = DowncastableWidget {
            type_id: widget.clone().lock().unwrap().type_id(),
            dyn_widget: widget,
        };
        self.widgets.insert(wid, dw);
    }

    pub fn get(&self, wid: usize) -> Option<DowncastableWidget> {
        self.widgets.get(&wid).cloned()
    }
}
