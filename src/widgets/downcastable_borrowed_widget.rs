use std::any::TypeId;
use std::sync::{Arc, Mutex};

use super::Widget;

pub type BorrowedInternalWidgetT<T> = Mutex<Box<T>>;
pub type BorrowedWidgetT<T> = Arc<BorrowedInternalWidgetT<T>>;
pub type BorrowedDynWidget = BorrowedWidgetT<dyn Widget>;
pub type OwnedDynWidget = Box<dyn Widget>;
pub type MutRefDynWidget<'a> = &'a mut Box<dyn Widget>;

#[derive(Clone, Debug)]
pub struct DowncastableBorrowedWidget {
    type_id: TypeId,
    borrowed_dyn_widget: BorrowedDynWidget,
    class: String,
}

impl DowncastableBorrowedWidget {
    pub fn new(
        type_id: TypeId,
        borrowed_dyn_widget: BorrowedDynWidget,
        class: &str,
    ) -> DowncastableBorrowedWidget {
        DowncastableBorrowedWidget {
            type_id,
            borrowed_dyn_widget,
            class: class.to_string(),
        }
    }
    pub fn bor_dyn_widget(&self) -> BorrowedDynWidget {
        self.borrowed_dyn_widget.clone()
    }
    pub fn widget_t<T: Widget>(&self) -> Option<BorrowedWidgetT<T>> {
        let ti = TypeId::of::<T>();
        if ti == self.type_id {
            let arc = Self::downcast_dyn_widget(self.borrowed_dyn_widget.clone());
            return Some(arc);
        }
        None
    }
    fn downcast_dyn_widget<T: Widget>(widget: BorrowedDynWidget) -> BorrowedWidgetT<T>
    where
        Self: Sized,
    {
        unsafe { Arc::from_raw(Arc::into_raw(widget) as *const BorrowedInternalWidgetT<T>) }
    }
    pub fn own_dyn_widget(self) -> BorrowedDynWidget {
        self.borrowed_dyn_widget
    }
    pub fn get_borrowed_strong_count(&self) -> usize {
        Arc::strong_count(&self.borrowed_dyn_widget)
    }
    pub fn class(&self) -> &str {
        &self.class
    }
}
