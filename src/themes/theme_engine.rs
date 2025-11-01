use std::collections::HashMap;
use std::{any::TypeId, fmt::Debug};

use crate::themes::Style;

use super::ThemeForWidget;

pub trait ThemeEngine: Debug + Sync + Send {
    fn default_style(&self) -> Vec<Box<dyn Style>>;
    fn get_themes(&self) -> HashMap<TypeId, &'static dyn ThemeForWidget>;
}
