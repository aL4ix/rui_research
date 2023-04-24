use crate::general::{Rect, Vector2D};
use crate::widgets::Primitive;
use mopa::{Any, mopafy};

pub trait Widget: Primitive + Any {
    fn event_mouse_button_down(&mut self, x: i32, y: i32);
    fn set_event_mouse_button_down(&mut self, callback: fn(this: &mut dyn Widget, x: i32, y: i32));
    fn get_rect(&self) -> Rect<f32> { // Maybe upgrade to Primitive?
        let (w, h) = self.size().unpack();
        let (x, y) = self.position().unpack();
        Rect::new(x, y, w, h)
    }
    fn accepts_mouse(&self, x: i32, y: i32) -> bool {
        self.get_rect().contains_point(Vector2D::<f32>::new(x as f32, y as f32))
    }
}

mopafy!(Widget);