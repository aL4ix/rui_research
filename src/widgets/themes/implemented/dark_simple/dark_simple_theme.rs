use std::fmt::Debug;

use crate::widgets::{
    themes::{ThemeEngine, ThemeForImage, ThemeForTextBox},
    Style, ThemeForButton, ThemeStyle,
};

use super::{
    DarkSimpleStyle, DarkSimpleThemeForButton, DarkSimpleThemeForImage, DarkSimpleThemeForTextBox,
};

#[derive(Debug)]
pub struct DarkSimpleTheme;

impl DarkSimpleTheme {}

impl ThemeEngine for DarkSimpleTheme {
    fn default_style(&self) -> Vec<Box<dyn Style>> {
        DarkSimpleStyle::default_style()
    }
    fn get_button_theme(&self) -> &dyn ThemeForButton {
        &DarkSimpleThemeForButton
    }
    fn get_text_box_theme(&self) -> &dyn ThemeForTextBox {
        &DarkSimpleThemeForTextBox
    }
    fn get_image_theme(&self) -> &dyn ThemeForImage {
        &DarkSimpleThemeForImage
    }
}
