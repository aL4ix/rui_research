use std::rc::Rc;

use glyph_brush::ab_glyph::FontArc;

use crate::general::{Color, Vector2D};
use crate::widgets::Primitive;
use crate::widgets::primitives::{Shape, Text};
use crate::widgets::themes::style::{ButtonStyle, Style, TextBoxStyle, ThemeButtonStyle};
use crate::widgets::themes::theme::{PrimitiveOrOneRef, Property, Theme};

pub struct SimpleTheme;

impl Theme for SimpleTheme {
    fn for_button(&self, size: Vector2D<f32>, text: &str, style: ThemeButtonStyle) -> (Vector2D<f32>, Vec<PrimitiveOrOneRef<Text>>, Rc<Text>) {
        let text_pri = Text::new(0, text, style.font_size, style.font, style.color);
        let text_size = text_pri.size().clone();
        let shape = Shape::new_square(0, text_size.clone(), 0, style.background_color);
        let text_rc = Rc::new(text_pri);
        use PrimitiveOrOneRef::*;
        (text_size, vec![Prim(Box::new(shape)), Ref(text_rc.clone())], text_rc)
    }
    fn for_textbox(&self, size: Vector2D<f32>, text: &str, font: FontArc, font_size: f32, color_text: Color, color_background: Color) -> (Vector2D<f32>, Vec<PrimitiveOrOneRef<Text>>, Rc<Text>) {
        let text_pri = Text::new(1, text, font_size, font, color_text);
        let text_size = text_pri.size().clone();
        let text_rc = Rc::new(text_pri);
        use PrimitiveOrOneRef::*;
        (text_size, vec![Ref(text_rc.clone())], text_rc)
    }
    fn style(&self) -> Vec<Box<dyn Style>> {
        vec![
            Box::new(ButtonStyle {
                color: vec![255, 0, 0],
                background_color: vec![0, 0, 255],
                font: "Nouveau_IBM".to_string(),
                font_size: 32.0,
                extra: vec![
                    ("color_background_gradient".to_string(), Property::Col(Color::new_opaque(0, 255, 0)))
                ],
                ..Default::default()
            }),
            Box::new(TextBoxStyle {
                color: vec![255, 0, 0],
                background_color: vec![0, 0, 255],
                font: "Nouveau_IBM".to_string(),
                font_size: 32.0,
                ..Default::default()
            }),
        ]
    }
}