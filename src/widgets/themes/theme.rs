use std::error::Error;
use std::path::Path;
use std::rc::Rc;

use crate::general::Vector2D;
use crate::widgets::primitives::{Bitmap, Text};
use crate::widgets::themes::style::{Style, ThemeButtonStyle, ThemeImageStyle, ThemeTextBoxStyle};
use crate::widgets::Primitive;

#[derive(Debug)]
pub enum PrimitiveOrOneRef<T> {
    Ref(Rc<T>),
    Prim(Box<dyn Primitive>),
}

// #[derive(Debug)]
// pub enum PrimitiveOrTwoRef<T1, T2> {
//     Ref1(Rc<T1>),
//     Ref2(Rc<T2>),
//     Prim(Box<dyn Primitive>),
// }

type SizePrimsAndRef<T> = (Vector2D<f32>, Vec<PrimitiveOrOneRef<T>>, Rc<T>);

pub trait Theme {
    fn for_button(
        &self,
        size: Vector2D<f32>,
        text: &str,
        style: ThemeButtonStyle,
    ) -> (Vector2D<f32>, Vec<PrimitiveOrOneRef<Text>>, Rc<Text>);
    fn for_text_box(
        &self,
        size: Vector2D<f32>,
        text: &str,
        style: ThemeTextBoxStyle,
    ) -> (Vector2D<f32>, Vec<PrimitiveOrOneRef<Text>>, Rc<Text>);
    fn for_image(
        &self,
        size: Vector2D<f32>,
        path: Box<Path>,
        style: ThemeImageStyle,
    ) -> Result<SizePrimsAndRef<Bitmap>, Box<dyn Error>>;
    fn style(&self) -> Vec<Box<dyn Style>>;
}
