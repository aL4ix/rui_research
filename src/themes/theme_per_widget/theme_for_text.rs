use crate::{
    general::Vector2D,
    themes::{PrimitivesManagerForThemes, ThemeStyleForTextBox},
};

use super::ThemeForWidget;

pub trait ThemeForTextBox: ThemeForWidget {
    fn new_text_box(
        &self,
        text: &str,
        size_for_clipping: Option<Vector2D<f32>>,
        style: Box<ThemeStyleForTextBox>,
        prim_man: &mut PrimitivesManagerForThemes,
    ) -> Vector2D<f32>;
    fn set_text(
        &self,
        text: &str,
        size_for_clipping: Option<Vector2D<f32>>,
        style: Box<ThemeStyleForTextBox>,
        prim_man: &mut PrimitivesManagerForThemes,
    ) -> Vector2D<f32>;
}
