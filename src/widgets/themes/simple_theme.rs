use std::error::Error;
use std::path::Path;
use std::rc::Rc;

use log::debug;

use crate::general::{Color, Vector2D};
use crate::widgets::primitives::{Bitmap, Shape, Text};
use crate::widgets::themes::style::{
    ButtonStyle, ImageStyle, Style, TextBoxStyle, ThemeButtonStyle, ThemeImageStyle,
    ThemeTextBoxStyle,
};
use crate::widgets::themes::theme::{PrimitiveOrOneRef, Theme};
use crate::widgets::Primitive;

pub struct SimpleTheme;

impl Theme for SimpleTheme {
    fn for_button(
        &self,
        _size: Vector2D<f32>,
        text: &str,
        style: ThemeButtonStyle,
    ) -> (Vector2D<f32>, Vec<PrimitiveOrOneRef<Text>>, Rc<Text>) {
        debug!("{:?}", style);
        let text_pri = Text::new(0, text, style.font_size, style.font, style.color);
        let text_size = text_pri.size().clone();
        let shape = Shape::new_square(0, text_size.clone(), 0, style.background_color);
        let text_rc = Rc::new(text_pri);
        use PrimitiveOrOneRef::*;
        (
            text_size,
            vec![Prim(Box::new(shape)), Ref(text_rc.clone())],
            text_rc,
        )
    }
    fn for_text_box(
        &self,
        _size: Vector2D<f32>,
        text: &str,
        style: ThemeTextBoxStyle,
    ) -> (Vector2D<f32>, Vec<PrimitiveOrOneRef<Text>>, Rc<Text>) {
        debug!("{:?}", style);
        let text_pri = Text::new(1, text, style.font_size, style.font, style.color);
        let text_size = text_pri.size().clone();
        let text_rc = Rc::new(text_pri);
        use PrimitiveOrOneRef::*;
        (text_size, vec![Ref(text_rc.clone())], text_rc)
    }
    fn for_image(
        &self,
        _size: Vector2D<f32>,
        path: Box<Path>,
        _style: ThemeImageStyle,
    ) -> Result<(Vector2D<f32>, Vec<PrimitiveOrOneRef<Bitmap>>, Rc<Bitmap>), Box<dyn Error>> {
        let bitmap = Bitmap::from_bmp(0, path)?;
        let size = bitmap.size().clone();
        let bitmap_rc = Rc::new(bitmap);
        use PrimitiveOrOneRef::*;
        Ok((size, vec![Ref(bitmap_rc.clone())], bitmap_rc))
    }

    fn style(&self) -> Vec<Box<dyn Style>> {
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
                color: (255, 0, 0, 255),
                background_color: (0, 0, 255, 255),
                font: "Nouveau_IBM".to_string(),
                font_size: 100.0,
                ..Default::default()
            }),
            Box::new(ImageStyle {
                ..Default::default()
            }),
        ]
    }
}
