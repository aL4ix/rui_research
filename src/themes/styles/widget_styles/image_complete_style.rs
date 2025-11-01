use std::error::Error;

use crate::themes::{ExtraStyleMap, PropertiesMap, StyleEnum, StyleExtractor, StyleForWidget};

#[derive(Debug)]
pub struct ThemeStyleForImage {
    pub extra: ExtraStyleMap,
}

impl StyleForWidget for ThemeStyleForImage {
    fn new(mut properties: PropertiesMap) -> Result<Self, Box<dyn Error>> {
        use StyleEnum::*;
        Ok(ThemeStyleForImage {
            extra: StyleExtractor.extract(&mut properties, &Extra)?.try_into()?,
        })
    }
}
