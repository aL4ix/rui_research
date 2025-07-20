use std::error::Error;

use glyph_brush::ab_glyph::FontArc;

use crate::{
    general::{self, Color, Vector2D},
    themes::{
        property::ApplyTo, ExtraStyle, ExtraStyleMap, GeneralStyleVec, PropertiesMap, Style,
        StyleEnum, StyleExtractor, StyleForWidget,
    },
};

#[derive(Debug, Default)]
pub struct TextBoxCompleteStyle {
    pub apply_to: ApplyTo,
    pub color: (u8, u8, u8, u8),
    pub background_color: (u8, u8, u8, u8),
    pub size: Option<(f32, f32)>,
    pub font: String,
    pub font_size: f32,
    pub extra: ExtraStyle,
}

impl Style for TextBoxCompleteStyle {
    fn to_properties_map(&self) -> PropertiesMap {
        use StyleEnum::*;
        let vec_for_map: GeneralStyleVec = vec![
            (ApplyTo, self.apply_to.clone().into()),
            (Color, general::Color::from(&self.color).into()),
            (
                BackgroundColor,
                general::Color::from(&self.background_color).into(),
            ),
            (Size, (&self.size).into()),
            (Font, self.font.clone().into()),
            (FontSize, self.font_size.into()),
            (Extra, self.extra.clone().into()),
        ];
        vec_for_map.into_iter().collect()
    }
}

#[derive(Debug)]
pub struct ThemeStyleForTextBox {
    pub color: Color,
    pub background_color: Color,
    pub size: Option<Vector2D<f32>>,
    pub font: FontArc,
    pub font_size: f32,
    pub extra: ExtraStyleMap,
}

impl StyleForWidget for ThemeStyleForTextBox {
    fn new(mut properties: PropertiesMap) -> Result<Self, Box<dyn Error>> {
        let p = &mut properties;
        let e = StyleExtractor;
        use StyleEnum::*;
        Ok(ThemeStyleForTextBox {
            color: e.extract(p, &Color)?.try_into()?,
            background_color: e.extract(p, &BackgroundColor)?.try_into()?,
            size: e.extract(p, &Size)?.try_into()?,
            font: e.extract(p, &Font)?.try_into()?,
            font_size: e.extract(p, &FontSize)?.try_into()?,
            extra: e.extract(p, &Extra)?.try_into()?,
        })
    }
}
