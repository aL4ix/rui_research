use std::fmt::Debug;

pub trait WidgetEnum: Clone + Copy + Debug {
    fn to_wid(self) -> usize;
}
