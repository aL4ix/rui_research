use std::error::Error;

use glyph_brush::ab_glyph::FontArc;

use crate::{
    general::{Color, Vector2D},
    themes::{ExtraStyleMap, PropertiesMap, StyleEnum, StyleExtractor, StyleForWidget},
};

use crate::{
    general,
    themes::{property::ApplyTo, ExtraStyle, GeneralStyleVec, Style},
};

#[derive(Debug, Default)]
pub struct ButtonCompleteStyle {
    pub apply_to: ApplyTo,
    pub color: (u8, u8, u8, u8),
    pub background_color: (u8, u8, u8, u8),
    pub size: Option<(f32, f32)>,
    pub font: String,
    pub font_size: f32,
    pub extra: ExtraStyle,
}

impl Style for ButtonCompleteStyle {
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
pub struct ThemeStyleForButton {
    pub color: Color,
    pub background_color: Color,
    pub size: Option<Vector2D<f32>>,
    pub font: FontArc,
    pub font_size: f32,
    pub extra: ExtraStyleMap,
}

impl StyleForWidget for ThemeStyleForButton {
    fn new(mut properties: PropertiesMap) -> Result<Box<dyn StyleForWidget>, Box<dyn Error>> {
        let p = &mut properties;
        let e = StyleExtractor;
        use StyleEnum::*;
        Ok(Box::new(ThemeStyleForButton {
            color: e.extract(p, &Color)?.try_into()?,
            background_color: e.extract(p, &BackgroundColor)?.try_into()?,
            size: e.extract(p, &Size)?.try_into()?,
            font: e.extract(p, &Font)?.try_into()?,
            font_size: e.extract(p, &FontSize)?.try_into()?,
            extra: e.extract(p, &Extra)?.try_into()?,
        }))
    }
}
