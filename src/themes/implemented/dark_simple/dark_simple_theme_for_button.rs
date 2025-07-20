use std::fmt::Debug;

use log::info;

use crate::themes::{
    PrimEnum, PrimId, PrimitivesManagerForThemes, ThemeForButton, ThemeForWidget,
    ThemeStyleForButton,
};
use crate::{
    general::Vector2D,
    utils::Downcast,
    widgets::primitives::{Primitive, Shape, Text},
};

#[repr(usize)]
#[derive(Clone, Copy, Debug)]
enum ButtonPrimEnum {
    Text,
    Square,
}

impl PrimEnum for ButtonPrimEnum {
    fn to_prim_id(self) -> PrimId {
        self as PrimId
    }
}

pub struct DarkSimpleThemeForButton;

impl ThemeForButton for DarkSimpleThemeForButton {
    fn new_button(
        &self,
        text: &str,
        size_for_clipping: Option<Vector2D<f32>>,
        style: Box<ThemeStyleForButton>,
        prim_man: &mut PrimitivesManagerForThemes,
    ) -> Vector2D<f32> {
        assert!(
            size_for_clipping.is_none(),
            "theme_for_button:ThemeForButton:new size_for_clipping not supported yet."
        );
        let mut text_prim = Text::new(0, text, style.font_size, style.font, style.color);
        let text_size = text_prim.size().clone();
        prim_man.insert(ButtonPrimEnum::Text, text_prim, 1);
        prim_man.insert(
            ButtonPrimEnum::Square,
            Shape::new_square(0, text_size.clone(), 0, style.background_color),
            0,
        );
        text_size
    }
    fn set_text(
        &self,
        text: &str,
        size_for_clipping: Option<Vector2D<f32>>,
        style: Box<ThemeStyleForButton>,
        prim_man: &mut PrimitivesManagerForThemes,
    ) -> Vector2D<f32> {
        assert!(
            size_for_clipping.is_none(),
            "theme_for_button:ThemeForButton:new"
        );
        info!("DaskSimpleThemeForButton:set_text {}", text);
        let prim_text = prim_man
            .get_mut(ButtonPrimEnum::Text)
            .expect("DarkSimpleThemeForButton:set_text get_mut");
        let text_prim = (**prim_text)
            .downcast_mut::<Text>()
            .expect("DarkSimpleThemeForButton:set_text downcast_mut");
        text_prim.set_text(text);
        let text_size = text_prim.size().clone();
        prim_man
            .remove(ButtonPrimEnum::Square)
            .expect("DarkSimpleThemeForButton:set_text remove");
        prim_man.insert(
            ButtonPrimEnum::Square,
            Shape::new_square(0, text_size.clone(), 0, style.background_color),
            0,
        );
        text_size
    }
}

impl ThemeForWidget for DarkSimpleThemeForButton {}
