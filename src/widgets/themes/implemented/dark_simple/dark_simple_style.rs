use crate::general::Color;
use crate::widgets::themes::property::ApplyTo;
use crate::widgets::ExtraStyleEnum::BackgroundColorGradient;
use crate::widgets::{
    ButtonCompleteStyle, GeneralStyle, Image, Primitive, Style, StyleEnum, TextBoxCompleteStyle,
    ThemeStyle,
};

pub struct DarkSimpleStyle;

impl ThemeStyle for DarkSimpleStyle {
    fn default_style() -> Vec<Box<dyn Style>> {
        vec![
            Box::new(ButtonCompleteStyle {
                color: (255, 255, 255, 255),
                background_color: (128, 128, 128, 255),
                font: "Nouveau_IBM".to_string(),
                font_size: 32.0,
                extra: vec![(BackgroundColorGradient, Color::new_opaque(0, 255, 0).into())],
                ..Default::default()
            }),
            Box::new(TextBoxCompleteStyle {
                color: (255, 255, 255, 255),
                background_color: (0, 0, 255, 255),
                font: "Nouveau_IBM".to_string(),
                font_size: 50.0,
                ..Default::default()
            }),
            Box::new(GeneralStyle {
                apply_to: ApplyTo::Class("Image".to_string()),
                style: vec![(StyleEnum::Class, Image::class_name().to_string().into())],
                ..Default::default()
            }),
        ]
    }
}
