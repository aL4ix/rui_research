use std::{any::Any, error::Error, fmt::Debug};

use crate::themes::{PropertiesMap, Property, StyleEnum};

pub trait StyleForWidget: Any + Debug {
    fn new(properties: PropertiesMap) -> Result<Box<dyn StyleForWidget>, Box<dyn Error>>
    where
        Self: Sized;
}

pub struct StyleExtractor;

impl StyleExtractor {
    pub fn extract(
        &self,
        properties: &mut PropertiesMap,
        style_enum: &StyleEnum,
    ) -> Result<Property, String> {
        properties
            .remove(style_enum)
            .ok_or(format!("No {:?} in properties map", style_enum))
    }
}
