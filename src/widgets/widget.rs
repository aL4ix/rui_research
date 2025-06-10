use log::debug;

use crate::general::{Rect, Vector2D};
use crate::widgets::Primitive;
use crate::window::Root;

use super::events::HasEvents;
use super::{BorrowedWidgetT, WidgetEnum};

pub type WidgetId = usize;

pub trait Widget: Primitive + HasEvents {
    fn get_rect(&mut self) -> Rect<f32> {
        // Maybe upgrade to Primitive?
        let (w, h) = self.size().unpack();
        let (x, y) = self.position().unpack();
        Rect::new(x, y, w, h)
    }
    fn will_accept_mouse_click_event(&mut self, x: i32, y: i32) -> bool {
        self.get_rect()
            .contains_point(Vector2D::<f32>::new(x as f32, y as f32))
    }
    fn get_by_id<WENUM: WidgetEnum>(
        root: &mut dyn Root,
        wenum: WENUM,
    ) -> Result<BorrowedWidgetT<Self>, String>
    where
        Self: Sized,
    {
        let wid = wenum.to_wid();
        debug!("Widget:get_by_id wid={}", wid);
        let option_dw = root.get_down_widget_by_id(wid);
        if let Some(dw) = option_dw {
            let option_wt = dw.widget_t::<Self>();
            match option_wt {
                Some(wt) => {
                    debug!("Could convert widget wid={} to {}", wid, Self::class_name());
                    return Ok(wt);
                }
                None => return Err(format!("get_by_id(): Not a {}", Self::class_name())),
            }
        }
        Err(format!("Not found: widget:Widget:get_by_id({})", wid))
    }
}
