use std::any::TypeId;
use std::cell::RefCell;
use std::rc::Rc;

use super::Widget;

pub type BorrowedInternalWidgetT<T> = RefCell<Box<T>>;
pub type BorrowedWidgetT<T> = Rc<BorrowedInternalWidgetT<T>>;
pub type BorrowedDynWidget = BorrowedWidgetT<dyn Widget>;
pub type OwnedDynWidget = Box<dyn Widget>;
pub type MutRefDynWidget<'a> = &'a mut Box<dyn Widget>;

#[derive(Clone, Debug)]
pub struct DowncastableBorrowedWidget {
    type_id: TypeId,
    borrowed_dyn_widget: BorrowedDynWidget,
}

impl DowncastableBorrowedWidget {
    pub fn new(
        type_id: TypeId,
        borrowed_dyn_widget: BorrowedDynWidget,
    ) -> DowncastableBorrowedWidget {
        DowncastableBorrowedWidget {
            type_id,
            borrowed_dyn_widget,
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
        let new_arc =
            unsafe { Rc::from_raw(Rc::into_raw(widget) as *const BorrowedInternalWidgetT<T>) };
        new_arc
    }
    pub fn own_dyn_widget(self) -> BorrowedDynWidget {
        return self.borrowed_dyn_widget;
    }
    pub fn get_borrowed_strong_count(&self) -> usize {
        Rc::strong_count(&self.borrowed_dyn_widget)
    }
}
