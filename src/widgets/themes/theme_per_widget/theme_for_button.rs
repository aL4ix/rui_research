use crate::{
    general::Vector2D,
    widgets::themes::{style::StyleForButton, PrimitiveManagerForThemes},
};

use super::ThemeForWidget;

pub trait ThemeForButton: ThemeForWidget {
    fn new(
        &self,
        text: &str,
        size_for_clipping: Option<Vector2D<f32>>,
        style: Box<StyleForButton>,
        prim_man: &mut PrimitiveManagerForThemes,
    ) -> Vector2D<f32>;
    fn set_text(
        &self,
        text: &str,
        size_for_clipping: Option<Vector2D<f32>>,
        style: Box<StyleForButton>,
        prim_man: &mut PrimitiveManagerForThemes,
    ) -> Vector2D<f32>;
}
