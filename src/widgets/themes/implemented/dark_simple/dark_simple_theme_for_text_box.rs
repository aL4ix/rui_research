use std::fmt::Debug;

use crate::{
    general::Vector2D,
    utils::Downcast,
    widgets::{
        primitives::Text,
        themes::{PrimEnum, PrimId, PrimitiveManagerForThemes, ThemeForTextBox, ThemeForWidget},
        Primitive, ThemeStyleForTextBox,
    },
};

#[repr(usize)]
#[derive(Clone, Copy, Debug)]
enum TextBoxPrimEnum {
    Text,
}

impl PrimEnum for TextBoxPrimEnum {
    fn to_prim_id(self) -> PrimId {
        self as PrimId
    }
}

pub struct DarkSimpleThemeForTextBox;

impl ThemeForTextBox for DarkSimpleThemeForTextBox {
    fn new_text_box(
        &self,
        text: &str,
        size_for_clipping: Option<Vector2D<f32>>,
        style: Box<ThemeStyleForTextBox>,
        prim_man: &mut PrimitiveManagerForThemes,
    ) -> Vector2D<f32> {
        assert!(
            size_for_clipping.is_none(),
            "theme_for_button:ThemeForTextBox:new size_for_clipping not supported yet."
        );
        let mut text_prim = Text::new(0, text, style.font_size, style.font, style.color);
        let text_size = text_prim.size().clone();
        prim_man.insert(TextBoxPrimEnum::Text, text_prim, 0);
        text_size
    }
    fn set_text(
        &self,
        text: &str,
        size_for_clipping: Option<Vector2D<f32>>,
        _style: Box<ThemeStyleForTextBox>,
        prim_man: &mut PrimitiveManagerForThemes,
    ) -> Vector2D<f32> {
        assert!(
            size_for_clipping.is_none(),
            "theme_for_button:ThemeForTextBox:new"
        );
        let prim_text = prim_man
            .get_mut(TextBoxPrimEnum::Text)
            .expect("DarkSimpleThemeForTextBox:set_text get_mut");
        let text_prim = (**prim_text)
            .downcast_mut::<Text>()
            .expect("DarkSimpleThemeForTextBox:set_text downcast_mut");
        text_prim.set_text(text);
        text_prim.size().clone()
    }
}

impl ThemeForWidget for DarkSimpleThemeForTextBox {}
