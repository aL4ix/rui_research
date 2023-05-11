use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use crate::widgets::events::Event;
use crate::window::Root;

pub type MouseButtonDownCallback = fn(this: &mut dyn Root, x: i32, y: i32);

pub struct MouseButtonDown {
    pub callback: Arc<MouseButtonDownCallback>,
}

impl MouseButtonDown {
    pub(crate) fn empty_callback(_this: &mut dyn Root, _x: i32, _y: i32) {}
}

impl Event for MouseButtonDown {
    fn class(&self) -> &str {
        "MouseButtonDown"
    }

    type Callback = MouseButtonDownCallback;

    fn clone_callback(&self) -> Arc<Self::Callback> {
        Arc::clone(&self.callback)
    }
}

impl Debug for MouseButtonDown {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.dyn_fmt(f)
    }
}

impl Default for MouseButtonDown {
    fn default() -> Self {
        MouseButtonDown {
            callback: Arc::new(Self::empty_callback)
        }
    }
}