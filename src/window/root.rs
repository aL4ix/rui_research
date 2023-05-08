use crate::widgets::Widget;

pub trait Root {
    fn get_widget_by_id(&mut self, id: usize) -> Option<&mut Box<dyn Widget>>;
}