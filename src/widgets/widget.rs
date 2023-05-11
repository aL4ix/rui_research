use std::error::Error;
use std::sync::Arc;

use mopa::{Any, mopafy};

use crate::general::{Rect, Vector2D};
use crate::widgets::events::MouseButtonDownCallback;
use crate::widgets::Primitive;
use crate::window::Root;

pub trait Widget: Primitive + Any {
    fn class_name() -> &'static str where Self: Sized;
    fn event_mouse_button_down(&self) -> Arc<MouseButtonDownCallback>;
    fn set_event_mouse_button_down(&mut self, callback: MouseButtonDownCallback);
    fn get_rect(&self) -> Rect<f32> { // Maybe upgrade to Primitive?
        let (w, h) = self.size().unpack();
        let (x, y) = self.position().unpack();
        Rect::new(x, y, w, h)
    }
    fn accepts_mouse(&self, x: i32, y: i32) -> bool {
        self.get_rect().contains_point(Vector2D::<f32>::new(x as f32, y as f32))
    }
    fn get_by_id(root: &mut dyn Root, id: usize) -> Result<&mut Self, Box<dyn Error>> where Self: Sized {
        if let Some(widget) = root.get_widget_by_id(id) {
            return if let Some(button) = widget.downcast_mut::<Self>() {
                Ok(button)
            } else {
                Err(Box::from(format!("Not a {}", Self::class_name())))
            };
        }
        Err(Box::from("Not found"))
    }
}

mopafy!(Widget);