use std::{
    any::{Any, TypeId},
    collections::HashMap,
    error::Error,
    sync::Arc,
};

use glyph_brush::ab_glyph::FontArc;
use log::debug;

use crate::{
    utils::Assets,
    widgets::{
        PropertiesMap, Property, StyleEnum, StyleForWidget, ThemeEngine, ThemeForWidget,
        ThemeStyleForButton, ThemeStyleForImage, ThemeStyleForTextBox, THEME_WIDGET_CAST_REGISTRY,
    },
};

#[derive(Debug)]
pub struct StyleMaster {
    _fonts: HashMap<String, FontArc>,
    styles: Vec<PropertiesMap>,
    theme_engine: Box<dyn ThemeEngine>,
}

impl StyleMaster {
    const COULD_NOT_FIND_THEME: &'static str = "Couldn't find theme for";
    const COULD_NOT_FIND_STYLE: &'static str = "Couldn't find style for";

    pub fn new(theme_engine: Box<dyn ThemeEngine>) -> Result<Arc<StyleMaster>, Box<dyn Error>> {
        let dyn_styles = theme_engine.default_style();
        let mut _fonts = HashMap::new();
        let mut styles = Vec::with_capacity(dyn_styles.len());

        for dyn_style in dyn_styles {
            debug!("{:?}", dyn_style);
            let mut prop_map = dyn_style.to_properties_map();

            // Convert fonts from string to Font
            if let Some(font_property) = prop_map.get(&StyleEnum::Font) {
                if let Property::Str(font_name) = font_property {
                    let maybe_font = _fonts.get(font_name);
                    let font_arc = match maybe_font {
                        None => {
                            let font_path = format!("assets/{}.ttf", font_name);
                            let font_vec = Assets::read(font_path)?;
                            let font_arc = FontArc::try_from_vec(font_vec)?;
                            _fonts.insert(font_name.to_string(), font_arc.clone());
                            font_arc
                        }
                        Some(font_arc) => font_arc.clone(),
                    };
                    prop_map.insert(StyleEnum::Font, Property::Font(font_arc));
                } else {
                    panic!("Font properties should only be Str: {:?}", prop_map);
                }
            }
            styles.push(prop_map);
        }

        Ok(Arc::new(StyleMaster {
            _fonts,
            styles,
            theme_engine,
        }))
    }
    pub fn theme_for_widget_t<T: ThemeForWidget + ?Sized>(&self, type_id: TypeId) -> Option<&T> {
        let opt_theme_widget = self.theme_engine.get_widget_theme_by_type(type_id);
        if let Some(theme_widget) = opt_theme_widget {
            let opt_theme_t: Option<&T> = (&*THEME_WIDGET_CAST_REGISTRY).cast_ref(theme_widget);
            if let Some(theme_t) = opt_theme_t {
                return Some(theme_t);
            }
        }
        None
    }
    pub fn expect_theme_for_widget_t<T: ThemeForWidget + ?Sized>(&self, type_id: TypeId) -> &T {
        self.theme_for_widget_t(type_id).expect(&format!(
            "{} {:?}!",
            Self::COULD_NOT_FIND_THEME,
            type_id
        ))
    }
    pub fn dyn_get_style(&self, type_id: &str) -> Result<Box<dyn StyleForWidget>, Box<dyn Error>> {
        let mut properties: PropertiesMap = Default::default();
        debug!("{:?}", self.styles);
        for style in &self.styles {
            if let Some(Property::Str(class)) = style.get(&StyleEnum::Class) {
                debug!("One retrieved style for class: {}", class);
                if class == type_id {
                    debug!("Found style");
                    properties.extend(style.into_iter().map(|(k, v)| (k.clone(), v.clone())));
                }
            }
        }
        debug!("Properties: {:?}", properties);
        debug!("type_id: {:?}", type_id);
        // TODO: Un-hardcode this
        let style_for_widget: Box<dyn StyleForWidget> = match type_id {
            "Button" => Box::new(ThemeStyleForButton::new(properties)?),
            "Image" => {
                debug!("Entered Image");
                Box::new(ThemeStyleForImage::new(properties)?)
            }
            "TextBox" => Box::new(ThemeStyleForTextBox::new(properties)?),
            _ => {
                debug!("Not found!");
                Err::<Box<dyn StyleForWidget>, Box<dyn Error>>(Box::from(format!(
                    "Unsupported type {:?}",
                    type_id
                )))?
            }
        };
        debug!("style_for_widget: {:?}", style_for_widget);
        Ok(style_for_widget)
    }
    pub fn style_for_widget_t<T: StyleForWidget>(&self, type_id: &str) -> Option<Box<T>> {
        let dyn_style_for_widget = self
            .dyn_get_style(type_id)
            .ok()
            .expect(&format!("Failed to retrieve a style for {:?}", type_id));
        return (dyn_style_for_widget as Box<dyn Any>).downcast().ok();
    }
    pub fn expect_style_for_widget_t<T: StyleForWidget>(&self, type_id: &str) -> Box<T> {
        self.style_for_widget_t(type_id).expect(&format!(
            "{} {}!",
            Self::COULD_NOT_FIND_STYLE,
            type_id
        ))
    }
}
