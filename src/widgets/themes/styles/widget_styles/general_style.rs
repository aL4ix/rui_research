use std::collections::HashMap;

use crate::widgets::{
    themes::property::ApplyTo, ExtraStyle, GeneralStyleVec, PropertiesMap, Property, Style,
    StyleEnum,
};

#[derive(Debug, Default)]
pub struct GeneralStyle {
    pub apply_to: ApplyTo,
    pub style: GeneralStyleVec,
    pub extra: ExtraStyle,
}

impl Style for GeneralStyle {
    fn to_properties_map(&self) -> PropertiesMap {
        let vec_for_map: GeneralStyleVec = vec![
            (StyleEnum::ApplyTo, self.apply_to.clone().into()),
            (StyleEnum::Extra, self.extra.clone().into()),
        ];
        let mut prop_map: HashMap<StyleEnum, Property> = vec_for_map.into_iter().collect();
        prop_map.extend(self.style.clone().into_iter());
        prop_map
    }
}
