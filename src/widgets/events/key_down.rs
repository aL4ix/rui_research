use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use sdl2::keyboard::Keycode;

use crate::widgets::events::Event;
use crate::window::Root;

pub type KeyDownCallback = fn(this: &mut dyn Root, key_code: Keycode);

pub struct KeyDown {
    pub callback: Arc<KeyDownCallback>,
}

impl KeyDown {
    pub(crate) fn empty_callback(_this: &mut dyn Root, _key_code: Keycode) {}
}

impl Event for KeyDown {
    fn class(&self) -> &str {
        "KeyDown"
    }

    type Callback = KeyDownCallback;

    fn clone_callback(&self) -> Arc<Self::Callback> {
        Arc::clone(&self.callback)
    }
}

impl Debug for KeyDown {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.dyn_fmt(f)
    }
}

impl Default for KeyDown {
    fn default() -> Self {
        KeyDown {
            callback: Arc::new(Self::empty_callback),
        }
    }
}
