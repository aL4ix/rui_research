use crate::widgets::{DowncastableBorrowedWidget, WidgetId};

pub trait Root {
    fn get_down_widget_by_id(&mut self, wid: WidgetId) -> Option<DowncastableBorrowedWidget>;
}
