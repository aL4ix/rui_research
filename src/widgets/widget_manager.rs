
use std::any::TypeId;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use log::debug;

use super::Widget;

pub type BorrowedInternalWidgetT<T> = Mutex<Box<T>>;
pub type BorrowedWidgetT<T> = Arc<BorrowedInternalWidgetT<T>>;
pub type BorrowedDynWidget = BorrowedWidgetT<dyn Widget>;
pub type OwnedDynWidget = Box<dyn Widget>;
pub type MutRefDynWidget<'a> = &'a mut Box<dyn Widget>;

pub trait WidgetEnum: Clone + Copy {
    fn to_isize(self) -> isize;
}

#[derive(Clone, Debug)]
pub struct DowncastableBorrowedWidget {
    type_id: TypeId,
    borrowed_dyn_widget: BorrowedDynWidget
}

impl DowncastableBorrowedWidget {
    pub fn bor_dyn_wid(&self) -> BorrowedDynWidget {
        self.borrowed_dyn_widget.clone()
    }
    pub fn wid_t<T: Widget>(&self) -> Option<BorrowedWidgetT<T>> {
        let ti = TypeId::of::<T>();
        if ti == self.type_id {
            let arc = Self::downcast_dyn_widget(self.borrowed_dyn_widget.clone());
            return Some(arc);
        }
        None
    }
    fn downcast_dyn_widget<T: Widget>(widget: BorrowedDynWidget) -> BorrowedWidgetT<T> where Self: Sized {
        let new_arc = unsafe { Arc::from_raw(Arc::into_raw(widget) as *const BorrowedInternalWidgetT<T>) };
        new_arc
    }
    pub fn own_dyn_wid(self) -> BorrowedDynWidget {
        return self.borrowed_dyn_widget;
    }
    pub fn get_borrowed_strong_count(&self) -> usize {
        Arc::strong_count(&self.borrowed_dyn_widget)
    }
}

#[derive(Default)]
pub struct WidgetManager {
    widgets: BTreeMap<usize, OwnedDynWidget>,
    borrowed: BTreeMap<usize, DowncastableBorrowedWidget>,
}

impl WidgetManager {
    pub fn insert(&mut self, wid: usize, widget: OwnedDynWidget) {
        self.widgets.insert(wid, widget);
    }
    pub fn down_borrow(&mut self, wid: usize) -> Option<DowncastableBorrowedWidget> {
        debug!("down_borrow wid={}", wid);

        if let Some(borrowed) = self.borrowed.get(&wid) {
            debug!("down_borrow strong_count={}", borrowed.get_borrowed_strong_count());
            return Some(borrowed.clone());
        }

        let opt_widget = self.widgets.remove(&wid);
        if let Some(widget) = opt_widget {
            let type_id = widget.type_id();
            let dyn_widget = Arc::new(Mutex::new(widget));
            let borrowed = DowncastableBorrowedWidget {
                type_id,
                borrowed_dyn_widget: dyn_widget,
            };
            self.borrowed.insert(wid, borrowed.clone());
            return Some(borrowed);
        }
        None
    }
    pub fn mut_ref(&mut self, wid: usize) -> Option<MutRefDynWidget> {
        // debug!("mut_ref wid={}", wid);
        self.widgets.get_mut(&wid)
    }
    pub fn ret_borrows(&mut self) {
        if self.borrowed.len() > 0 {
            debug!("ret_borrows len={}", self.borrowed.len());
        }
        
        for wid in self.borrowed.keys().cloned().collect::<Vec<_>>() {
            let down_borrow = self.borrowed.remove(&wid).expect("widget_manager:WidgetManager:ret_borrows remove");
            let dyn_wid = down_borrow.own_dyn_wid();
            let count = Arc::strong_count(&dyn_wid);
            debug!("wid={} count={}", wid, count);
            if count > 1 {
                panic!("Expected wid={} to have strong_count of 1, found {}", wid, count);
            }
            let mutex = Arc::try_unwrap(dyn_wid).expect("widget_manager:WidgetManager:ret_borrows Arc::try_unwrap");
            let widget = mutex.into_inner().expect("widget_manager:WidgetManager:ret_borrows into_inner");
            debug!("{}" ,widget.class());
            self.widgets.insert(wid, widget);
        }
    }
}
