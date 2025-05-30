use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
use std::sync::Arc;

use glyph_brush::ab_glyph::FontArc;
use log::debug;

use crate::general::{Color, Vector2D};
use crate::utils::Assets;
use crate::widgets::themes::Property;
use crate::widgets::Primitive;
use crate::widgets::*;

use super::ThemeEngine;

type PropertiesMap = HashMap<String, Property>;

pub trait Style: Debug {
    fn to_properties_map(&self) -> PropertiesMap;
}

#[derive(Debug, Clone)]
pub enum ApplyTo {
    Id(usize),
    Class(String),
    Groups(Vec<String>),
}

impl Default for ApplyTo {
    fn default() -> Self {
        ApplyTo::Id(0)
    }
}

#[derive(Debug, Default)]
pub struct ButtonStyle {
    pub apply_to: ApplyTo,
    pub color: (u8, u8, u8, u8),
    pub background_color: (u8, u8, u8, u8),
    pub size: Option<(f32, f32)>,
    pub font: String,
    pub font_size: f32,
    pub extra: Vec<(String, Property)>,
}

impl Style for ButtonStyle {
    fn to_properties_map(&self) -> PropertiesMap {
        let s = |s: &str| s.to_string();
        let mut r = HashMap::new();
        // TODO change to macro
        r.insert(s("class"), Button::class_name().to_string().into());
        r.insert(s("apply_to"), self.apply_to.clone().into());
        r.insert(s("color"), Color::from(&self.color).into());
        r.insert(
            s("background_color"),
            Color::from(&self.background_color).into(),
        );
        r.insert(s("size"), (&self.size).into());
        r.insert(s("font"), self.font.clone().into());
        r.insert(s("font_size"), self.font_size.into());
        r.extend(self.extra.clone());
        r
    }
}

#[derive(Debug, Default)]
pub struct TextBoxStyle {
    pub apply_to: ApplyTo,
    pub color: (u8, u8, u8, u8),
    pub background_color: (u8, u8, u8, u8),
    pub size: Option<(f32, f32)>,
    pub font: String,
    pub font_size: f32,
    pub extra: Vec<(String, Property)>,
}

impl Style for TextBoxStyle {
    fn to_properties_map(&self) -> PropertiesMap {
        let s = |s: &str| s.to_string();
        let mut r = HashMap::new();
        // TODO change to macro
        r.insert(s("class"), TextBox::class_name().to_string().into());
        r.insert(s("apply_to"), self.apply_to.clone().into());
        r.insert(s("color"), Color::from(&self.color).into());
        r.insert(
            s("background_color"),
            Color::from(&self.background_color).into(),
        );
        r.insert(s("size"), (&self.size).into());
        r.insert(s("font"), self.font.clone().into());
        r.insert(s("font_size"), self.font_size.into());
        r.extend(self.extra.clone());
        r
    }
}

#[derive(Debug, Default)]
pub struct ImageStyle {
    pub apply_to: ApplyTo,
    pub size: Option<(f32, f32)>,
    pub extra: Vec<(String, Property)>,
}

impl Style for ImageStyle {
    fn to_properties_map(&self) -> PropertiesMap {
        let s = |s: &str| s.to_string();
        let mut r = HashMap::new();
        // TODO change to macro
        r.insert(s("class"), Image::class_name().to_string().into());
        r.insert(s("apply_to"), self.apply_to.clone().into());
        r.insert(s("size"), (&self.size).into());
        r.extend(self.extra.clone());
        r
    }
}

pub trait StyleForWidget: Any {
    fn new(properties: PropertiesMap) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct GenericStyle {
    pub properties: PropertiesMap,
}

impl StyleForWidget for GenericStyle {
    fn new(properties: PropertiesMap) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        Ok(GenericStyle { properties })
    }
}

#[derive(Debug)]
pub struct StyleForButton {
    pub color: Color,
    pub background_color: Color,
    pub size: Option<Vector2D<f32>>,
    pub font: FontArc,
    pub font_size: f32,
    pub extra: HashMap<String, Property>,
}

impl StyleForWidget for StyleForButton {
    fn new(mut properties: PropertiesMap) -> Result<Self, Box<dyn Error>> {
        // TODO change to macro
        let color = properties
            .remove("color")
            .ok_or("No color")?
            .try_into()
            .map_err(|e| e + " into color")?;
        let background_color = properties
            .remove("background_color")
            .ok_or("No background_color")?
            .try_into()
            .map_err(|e| e + " into background_color")?;
        let size = properties
            .remove("size")
            .ok_or("No size")?
            .try_into()
            .map_err(|e| e + " into size")?;
        let font = properties
            .remove("font")
            .ok_or("No font")?
            .try_into()
            .map_err(|e| e + " into font")?;
        let font_size = properties
            .remove("font_size")
            .ok_or("No font_size")?
            .try_into()
            .map_err(|e| e + " into font_size")?;
        Ok(StyleForButton {
            color,
            background_color,
            size,
            font,
            font_size,
            extra: properties,
        })
    }
}

#[derive(Debug)]
pub struct StyleForTextBox {
    pub color: Color,
    pub background_color: Color,
    pub size: Option<Vector2D<f32>>,
    pub font: FontArc,
    pub font_size: f32,
    pub extra: HashMap<String, Property>,
}

impl StyleForWidget for StyleForTextBox {
    fn new(mut properties: PropertiesMap) -> Result<Self, Box<dyn Error>> {
        // TODO change to macro
        let color = properties
            .remove("color")
            .ok_or("No color")?
            .try_into()
            .map_err(|e| e + " into color")?;
        let background_color = properties
            .remove("background_color")
            .ok_or("No background_color")?
            .try_into()
            .map_err(|e| e + " into background_color")?;
        let size = properties
            .remove("size")
            .ok_or("No size")?
            .try_into()
            .map_err(|e| e + " into size")?;
        let font = properties
            .remove("font")
            .ok_or("No font")?
            .try_into()
            .map_err(|e| e + " into font")?;
        let font_size = properties
            .remove("font_size")
            .ok_or("No font_size")?
            .try_into()
            .map_err(|e| e + " into font_size")?;
        Ok(StyleForTextBox {
            color,
            background_color,
            size,
            font,
            font_size,
            extra: properties,
        })
    }
}

#[derive(Debug)]
pub struct StyleForImage {
    pub extra: HashMap<String, Property>,
}

impl StyleForWidget for StyleForImage {
    fn new(properties: PropertiesMap) -> Result<Self, Box<dyn Error>> {
        Ok(StyleForImage { extra: properties })
    }
}

#[derive(Debug)]
pub struct StyleMaster {
    _fonts: HashMap<String, FontArc>,
    styles: Vec<PropertiesMap>,
    theme_engine: Box<dyn ThemeEngine>,
}

impl StyleMaster {
    const FONT: &'static str = "font";
    const CLASS: &'static str = "class";
    const COULD_NOT_FIND_STYLE: &'static str = "Couldn't find style for";
    const COULD_NOT_FIND_THEME: &'static str = "Couldn't find theme for";

    pub fn new(theme_engine: Box<dyn ThemeEngine>) -> Result<Arc<StyleMaster>, Box<dyn Error>> {
        let dyn_styles = theme_engine.default_style();
        let mut fonts = HashMap::new();
        let mut styles = Vec::with_capacity(dyn_styles.len());
        for dyn_style in dyn_styles {
            debug!("{:?}", dyn_style);
            let mut map = dyn_style.to_properties_map();
            if let Some(font_property) = map.get(Self::FONT) {
                if let Property::Str(font_name) = font_property {
                    let maybe_font = fonts.get(font_name);
                    let font_arc = match maybe_font {
                        None => {
                            let font_path = format!("assets/{}.ttf", font_name);
                            let font_vec = Assets::read(font_path)?;
                            let font_arc = FontArc::try_from_vec(font_vec)?;
                            fonts.insert(font_name.to_string(), font_arc.clone());
                            font_arc
                        }
                        Some(font_arc) => font_arc.clone(),
                    };
                    map.insert(Self::FONT.to_string(), Property::Font(font_arc));
                } else {
                    panic!("Font properties should only be Str: {:?}", map);
                }
            }
            styles.push(map);
        }
        Ok(Arc::new(StyleMaster {
            _fonts: fonts,
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
        for style in &self.styles {
            if let Some(Property::Str(class)) = style.get(Self::CLASS) {
                if class == type_id {
                    properties.extend(style.into_iter().map(|(k, v)| (k.clone(), v.clone())));
                }
            }
        }
        // TODO: Un-hardcode this
        let style_for_widget: Box<dyn StyleForWidget> = match type_id {
            "Button" => Box::new(StyleForButton::new(properties)?),
            "Image" => Box::new(StyleForImage::new(properties)?),
            "TextBox" => Box::new(StyleForTextBox::new(properties)?),
            _ => Err::<Box<dyn StyleForWidget>, Box<dyn Error>>(Box::from(format!(
                "Unsupported type {:?}",
                type_id
            )))?,
        };
        Ok(style_for_widget)
    }
    pub fn style_for_widget_t<T: StyleForWidget>(&self, type_id: &str) -> Option<Box<T>> {
        let dyn_style_for_widget = self
            .dyn_get_style(type_id)
            .ok()
            .expect("Failed to retrieve a style");
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
