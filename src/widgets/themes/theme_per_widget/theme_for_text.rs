use crate::{
    general::Vector2D,
    widgets::{themes::PrimitiveManagerForThemes, ThemeStyleForTextBox},
};

use super::ThemeForWidget;

pub trait ThemeForTextBox: ThemeForWidget {
    fn new(
        &self,
        text: &str,
        size_for_clipping: Option<Vector2D<f32>>,
        style: Box<ThemeStyleForTextBox>,
        prim_man: &mut PrimitiveManagerForThemes,
    ) -> Vector2D<f32>;
    fn set_text(
        &self,
        text: &str,
        size_for_clipping: Option<Vector2D<f32>>,
        style: Box<ThemeStyleForTextBox>,
        prim_man: &mut PrimitiveManagerForThemes,
    ) -> Vector2D<f32>;
}
