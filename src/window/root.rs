use std::collections::BTreeMap;

use crate::widgets::{DowncastableBorrowedWidget, OwnedDynWidget, WidgetId};

pub trait Root {
    fn get_down_widget_by_id(&mut self, wid: WidgetId) -> Option<DowncastableBorrowedWidget>;
    fn children(&self) -> &BTreeMap<WidgetId, OwnedDynWidget>;
}
