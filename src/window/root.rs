
use crate::widgets::DowncastableBorrowedWidget;

pub trait Root {
    fn get_down_widget_by_id(&mut self, wid: usize) -> Option<DowncastableBorrowedWidget>;
}
