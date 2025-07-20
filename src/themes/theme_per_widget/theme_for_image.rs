use crate::{
    general::Vector2D,
    themes::{PrimitivesManagerForThemes, ThemeStyleForImage},
};
use std::path::Path;

use super::ThemeForWidget;

pub trait ThemeForImage: ThemeForWidget {
    fn new_image(
        &self,
        path: Box<Path>,
        size_for_clipping: Option<Vector2D<f32>>,
        style: Box<ThemeStyleForImage>,
        prim_man: &mut PrimitivesManagerForThemes,
    ) -> Vector2D<f32>;
}
