use std::{any::TypeId, collections::HashMap, fmt::Debug};

use crosstrait::entry;

use crate::{
    themes::{
        CrossTraitEntry, Style, ThemeEngine, ThemeForButton, ThemeForImage, ThemeForTextBox,
        ThemeForWidget, ThemeStyle,
    },
    widgets::{Image, TextBox},
};

use super::{
    DarkSimpleStyle, DarkSimpleThemeForButton, DarkSimpleThemeForImage, DarkSimpleThemeForTextBox,
};

use crate::widgets::Button;

#[derive(Debug)]
pub struct DarkSimpleTheme;

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
    fn get_crosstrait_registry(&self) -> &'static [CrossTraitEntry] {
        &[
            entry!(DarkSimpleThemeForButton => dyn ThemeForButton),
            entry!(DarkSimpleThemeForImage => dyn ThemeForImage),
            entry!(DarkSimpleThemeForTextBox => dyn ThemeForTextBox),
        ]
    }
}
