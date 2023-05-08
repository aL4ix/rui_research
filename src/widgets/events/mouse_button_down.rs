use std::fmt::{Debug, Formatter};

use crate::widgets::events::Event;
use crate::window::Root;

pub struct MouseButtonDown {
    pub callback: fn(this: &mut dyn Root, x: i32, y: i32),
}

impl MouseButtonDown {
    pub(crate) fn empty_callback(_this: &mut dyn Root, _x: i32, _y: i32) {}
}

impl Event for MouseButtonDown {
    fn class(&self) -> &str {
        "MouseButtonDown"
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
            callback: Self::empty_callback
        }
    }
}