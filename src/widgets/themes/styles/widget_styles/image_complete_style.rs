use std::error::Error;

use crate::widgets::{ExtraStyleMap, PropertiesMap, StyleEnum, StyleExtractor, StyleForWidget};

#[derive(Debug)]
pub struct ThemeStyleForImage {
    pub extra: ExtraStyleMap,
}

impl StyleForWidget for ThemeStyleForImage {
    fn new(mut properties: PropertiesMap) -> Result<Self, Box<dyn Error>> {
        let p = &mut properties;
        let e = StyleExtractor;
        use StyleEnum::*;
        Ok(ThemeStyleForImage {
            extra: e.extract(p, &Extra)?.try_into()?,
        })
    }
}
