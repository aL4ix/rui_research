use std::fmt::Debug;

use super::WidgetId;

pub trait WidgetEnum: Clone + Copy + Debug {
    fn to_wid(self) -> WidgetId;
}
