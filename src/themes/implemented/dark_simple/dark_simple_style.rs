use std::any::TypeId;

use crate::general::Color;
use crate::themes::property::ApplyTo::Type;
use crate::themes::ExtraStyleEnum::BackgroundColorGradient;
use crate::themes::{ButtonCompleteStyle, GeneralStyle, Style, TextBoxCompleteStyle, ThemeStyle};
use crate::widgets::{Button, Image, TextBox};

pub struct DarkSimpleStyle;

impl ThemeStyle for DarkSimpleStyle {
    fn default_style() -> Vec<Box<dyn Style>> {
        vec![
            Box::new(ButtonCompleteStyle {
                apply_to: Type(TypeId::of::<Button>()),
                color: (255, 255, 255, 255),
                background_color: (128, 128, 128, 255),
                font: "Nouveau_IBM".to_string(),
                font_size: 32.0,
                extra: vec![(BackgroundColorGradient, Color::new_opaque(0, 255, 0).into())],
                ..Default::default()
            }),
            Box::new(TextBoxCompleteStyle {
                apply_to: Type(TypeId::of::<TextBox>()),
                color: (255, 255, 255, 255),
                background_color: (0, 0, 255, 255),
                font: "Nouveau_IBM".to_string(),
                font_size: 50.0,
                ..Default::default()
            }),
            Box::new(GeneralStyle {
                apply_to: Type(TypeId::of::<Image>()),
                ..Default::default()
            }),
        ]
    }
}
