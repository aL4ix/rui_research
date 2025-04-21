use std::any::Any;
use std::sync::Arc;

use crate::general::{Rect, Vector2D};
use crate::widgets::events::MouseButtonDownCallback;
use crate::widgets::Primitive;
use crate::window::Root;

use super::WidgetT;

pub trait Widget: Primitive + Any {
    fn event_mouse_button_down(&self) -> Arc<MouseButtonDownCallback>;
    fn set_event_mouse_button_down(&mut self, callback: MouseButtonDownCallback);
    fn get_rect(&self) -> Rect<f32> {
        // Maybe upgrade to Primitive?
        let (w, h) = self.size().unpack();
        let (x, y) = self.position().unpack();
        Rect::new(x, y, w, h)
    }
    fn accepts_mouse(&self, x: i32, y: i32) -> bool {
        self.get_rect()
            .contains_point(Vector2D::<f32>::new(x as f32, y as f32))
    }
    fn get_by_id(root: &mut dyn Root, wid: usize) -> Result<WidgetT<Self>, String>
    where
        Self: Sized,
    {
        let option_dw = root.get_down_widget_by_id(wid);
        if let Some(dw) = option_dw {
            let option_wt = dw.wid_t::<Self>();
            match option_wt {
                Some(wt) => return Ok(wt),
                None => {
                    return Err(String::from(format!(
                        "get_by_id(): Not a {}",
                        Self::class_name()
                    )))
                }
            }
        }
        Err(String::from("Not found: get_by_id()"))
    }
}
