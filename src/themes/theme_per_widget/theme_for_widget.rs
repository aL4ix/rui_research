use std::any::Any;

use crosstrait::{entry, Registry};
use once_cell::sync::Lazy;

use crate::themes::{DarkSimpleThemeForButton, DarkSimpleThemeForImage, DarkSimpleThemeForTextBox};

use super::{ThemeForButton, ThemeForImage, ThemeForTextBox};

pub trait ThemeForWidget: Any {}

// TODO: Change this when a better library exists
pub static THEME_WIDGET_CAST_REGISTRY: Lazy<Registry> = Lazy::new(|| {
    Registry::new(&[
        entry!(DarkSimpleThemeForButton => dyn ThemeForButton),
        entry!(DarkSimpleThemeForImage => dyn ThemeForImage),
        entry!(DarkSimpleThemeForTextBox => dyn ThemeForTextBox),
    ])
});
