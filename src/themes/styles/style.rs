use std::collections::HashMap;
use std::fmt::Debug;

use crate::themes::{ExtraStyleEnum, Property, StyleEnum};

pub type PropertiesMap = HashMap<StyleEnum, Property>;
pub type ExtraStyle = Vec<(ExtraStyleEnum, Property)>;
pub type ExtraStyleMap = HashMap<ExtraStyleEnum, Property>;
pub type GeneralStyleVec = Vec<(StyleEnum, Property)>;

pub trait Style: Debug {
    fn to_properties_map(&self) -> PropertiesMap;
}
