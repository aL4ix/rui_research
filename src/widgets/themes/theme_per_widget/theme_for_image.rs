use crate::{
    general::Vector2D,
    widgets::themes::{style::StyleForImage, PrimitiveManagerForThemes},
};
use std::path::Path;

use super::ThemeForWidget;

pub trait ThemeForImage: ThemeForWidget {
    fn new(
        &self,
        path: Box<Path>,
        size_for_clipping: Option<Vector2D<f32>>,
        style: Box<StyleForImage>,
        prim_man: &mut PrimitiveManagerForThemes,
    ) -> Vector2D<f32>;
}
