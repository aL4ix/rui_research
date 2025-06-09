use std::{any::TypeId, fmt::Debug};

use crate::widgets::{Button, Image, Style, TextBox};

use super::{ThemeForButton, ThemeForImage, ThemeForTextBox, ThemeForWidget};

pub trait ThemeEngine: Debug + Sync + Send {
    fn get_widget_theme_by_type(&self, type_id: TypeId) -> Option<&dyn ThemeForWidget> {
        // TODO: how can we do this more dynamic, a static hashmap?
        if TypeId::of::<Button>() == type_id {
            return Some(self.get_button_theme());
        }
        if TypeId::of::<TextBox>() == type_id {
            return Some(self.get_text_box_theme());
        }
        if TypeId::of::<Image>() == type_id {
            return Some(self.get_image_theme());
        }
        println!("ThemeEngine found nothing for type {:?}", type_id);
        None
    }
    fn default_style(&self) -> Vec<Box<dyn Style>>;
    fn get_button_theme(&self) -> &dyn ThemeForButton;
    fn get_text_box_theme(&self) -> &dyn ThemeForTextBox;
    fn get_image_theme(&self) -> &dyn ThemeForImage;
}
