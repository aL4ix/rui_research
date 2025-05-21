
use crate::widgets::{DowncastableBorrowedWidget, BorrowedDynWidget};

pub trait Root {
    // fn get_widget_by_id_dyn(&mut self, wid: usize) -> Option<BorrowedDynWidget>;
    fn get_down_widget_by_id(&mut self, wid: usize) -> Option<DowncastableBorrowedWidget>;
}
