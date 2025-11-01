use std::any::Any;

use std::fmt::Debug;

pub trait ThemeForWidget: Any + Debug + Sync {}
