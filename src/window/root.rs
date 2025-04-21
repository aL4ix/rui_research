
use crate::widgets::{DowncastableWidget, DynWidget};

pub trait Root {
    fn get_widget_by_id_dyn(&mut self, wid: usize) -> Option<DynWidget>;
    fn get_down_widget_by_id(&mut self, wid: usize) -> Option<DowncastableWidget>;
}
