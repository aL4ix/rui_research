use crate::{
    general::Color,
    widgets::{
        themes::{
            style::{ButtonStyle, ImageStyle, TextBoxStyle},
            ThemeStyle,
        },
        Style,
    },
};

pub struct DarkSimpleStyle;

impl ThemeStyle for DarkSimpleStyle {
    fn default_style() -> Vec<Box<dyn Style>> {
        vec![
            Box::new(ButtonStyle {
                color: (255, 255, 255, 255),
                background_color: (128, 128, 128, 255),
                font: "Nouveau_IBM".to_string(),
                font_size: 32.0,
                extra: vec![(
                    "color_background_gradient".to_string(),
                    Color::new_opaque(0, 255, 0).into(),
                )],
                ..Default::default()
            }),
            Box::new(TextBoxStyle {
                color: (255, 255, 255, 255),
                background_color: (0, 0, 255, 255),
                font: "Nouveau_IBM".to_string(),
                font_size: 50.0,
                ..Default::default()
            }),
            Box::new(ImageStyle {
                ..Default::default()
            }),
        ]
    }
}
