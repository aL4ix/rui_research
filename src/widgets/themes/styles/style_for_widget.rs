use std::{any::Any, error::Error};

use crate::widgets::{PropertiesMap, Property, StyleEnum};

pub trait StyleForWidget: Any + std::fmt::Debug {
    fn new(properties: PropertiesMap) -> Result<Self, Box<dyn Error>>
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
