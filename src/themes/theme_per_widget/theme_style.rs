use crate::themes::Style;

pub trait ThemeStyle {
    fn default_style() -> Vec<Box<dyn Style>>
    where
        Self: Sized;
}
