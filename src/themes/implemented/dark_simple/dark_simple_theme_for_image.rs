use std::{fmt::Debug, path::Path};

use crate::{
    general::Vector2D,
    themes::{
        PrimEnum, PrimId, PrimitivesManagerForThemes, ThemeForImage, ThemeForWidget,
        ThemeStyleForImage,
    },
    widgets::primitives::{Bitmap, Primitive},
};

#[repr(usize)]
#[derive(Clone, Copy, Debug)]
enum ImagePrimEnum {
    Image,
}

impl PrimEnum for ImagePrimEnum {
    fn to_prim_id(self) -> PrimId {
        self as PrimId
    }
}

#[derive(Debug)]
pub struct DarkSimpleThemeForImage;

impl ThemeForImage for DarkSimpleThemeForImage {
    fn new_image(
        &self,
        path: Box<Path>,
        size_for_clipping: Option<Vector2D<f32>>,
        _style: Box<ThemeStyleForImage>,
        prim_man: &mut PrimitivesManagerForThemes,
    ) -> Vector2D<f32> {
        assert!(
            size_for_clipping.is_none(),
            "theme_for_button:ThemeForImage:new size_for_clipping not supported yet."
        );
        let mut bitmap = Bitmap::from_bmp(0, path).expect(stringify!(ThemeForImage));
        let size = bitmap.size().clone();
        prim_man.insert(ImagePrimEnum::Image, bitmap, 0);
        size
    }
}

impl ThemeForWidget for DarkSimpleThemeForImage {}
