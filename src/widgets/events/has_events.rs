use std::sync::Arc;

use super::MouseButtonDownCallback;

use super::KeyDownCallback;

pub trait HasEvents {
    fn event_mouse_button_down(&self) -> Arc<MouseButtonDownCallback>;
    fn set_event_mouse_button_down(&mut self, callback: MouseButtonDownCallback);
    fn event_key_down(&self) -> Arc<KeyDownCallback>;
    fn set_event_key_down(&mut self, callback: KeyDownCallback);
}
