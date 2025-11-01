use std::{any::TypeId, collections::HashMap, fmt::Debug};

use crate::{
    themes::{Style, ThemeEngine, ThemeForWidget, ThemeStyle},
    widgets::{Image, TextBox},
};

use super::{
    DarkSimpleStyle, DarkSimpleThemeForButton, DarkSimpleThemeForImage, DarkSimpleThemeForTextBox,
};

use crate::widgets::Button;

#[derive(Debug)]
pub struct DarkSimpleTheme;

impl DarkSimpleTheme {}

impl ThemeEngine for DarkSimpleTheme {
    fn default_style(&self) -> Vec<Box<dyn Style>> {
        DarkSimpleStyle::default_style()
    }
    fn get_themes(&self) -> HashMap<TypeId, &'static dyn ThemeForWidget> {
        HashMap::from([
            (
                TypeId::of::<Button>(),
                &DarkSimpleThemeForButton as &dyn ThemeForWidget,
            ),
            (TypeId::of::<TextBox>(), &DarkSimpleThemeForTextBox),
            (TypeId::of::<Image>(), &DarkSimpleThemeForImage),
        ])
    }
}
