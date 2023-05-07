use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
use std::path::Path;
use std::rc::Rc;

use glyph_brush::ab_glyph::FontArc;
use log::debug;

use crate::general::{Color, Vector2D};
use crate::utils::Assets;
use crate::widgets::*;
use crate::widgets::Primitive;
use crate::widgets::themes::Property;
use crate::widgets::themes::theme::{PrimitiveOrOneRef, Theme};

type PropertiesMap = HashMap<String, Property>;

pub trait Style: Debug {
    fn to_properties_map(&self) -> PropertiesMap;
}

#[allow(dead_code)]
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
        r.insert(s("background_color"), Color::from(&self.background_color).into());
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
        r.insert(s("background_color"), Color::from(&self.background_color).into());
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


trait ThemeStyle {
    fn new(properties: PropertiesMap) -> Result<Self, Box<dyn Error>> where Self: Sized;
}

#[derive(Debug)]
pub struct ThemeButtonStyle {
    pub color: Color,
    pub background_color: Color,
    pub size: Option<Vector2D<f32>>,
    pub font: FontArc,
    pub font_size: f32,
    pub extra: HashMap<String, Property>,
}

impl ThemeStyle for ThemeButtonStyle {
    fn new(mut properties: PropertiesMap) -> Result<Self, Box<dyn Error>> {
        // TODO change to macro
        let color = properties.remove("color").ok_or("No color")?.try_into().map_err(|e| e + " into color")?;
        let background_color = properties.remove("background_color").ok_or("No background_color")?.try_into().map_err(|e| e + " into background_color")?;
        let size = properties.remove("size").ok_or("No size")?.try_into().map_err(|e| e + " into size")?;
        let font = properties.remove("font").ok_or("No font")?.try_into().map_err(|e| e + " into font")?;
        let font_size = properties.remove("font_size").ok_or("No font_size")?.try_into().map_err(|e| e + " into font_size")?;
        Ok(ThemeButtonStyle {
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
pub struct ThemeTextBoxStyle {
    pub color: Color,
    pub background_color: Color,
    pub size: Option<Vector2D<f32>>,
    pub font: FontArc,
    pub font_size: f32,
    pub extra: HashMap<String, Property>,
}

impl ThemeStyle for ThemeTextBoxStyle {
    fn new(mut properties: PropertiesMap) -> Result<Self, Box<dyn Error>> {
        // TODO change to macro
        let color = properties.remove("color").ok_or("No color")?.try_into().map_err(|e| e + " into color")?;
        let background_color = properties.remove("background_color").ok_or("No background_color")?.try_into().map_err(|e| e + " into background_color")?;
        let size = properties.remove("size").ok_or("No size")?.try_into().map_err(|e| e + " into size")?;
        let font = properties.remove("font").ok_or("No font")?.try_into().map_err(|e| e + " into font")?;
        let font_size = properties.remove("font_size").ok_or("No font_size")?.try_into().map_err(|e| e + " into font_size")?;
        Ok(ThemeTextBoxStyle {
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
pub struct ThemeImageStyle {
    pub extra: HashMap<String, Property>,
}

impl ThemeStyle for ThemeImageStyle {
    fn new(properties: PropertiesMap) -> Result<Self, Box<dyn Error>> {
        Ok(ThemeImageStyle {
            extra: properties,
        })
    }
}


pub struct StyleMaster {
    _fonts: HashMap<String, FontArc>,
    styles: Vec<PropertiesMap>,
    theme: Box<dyn Theme>,
}

type OneWidget = (Vector2D<f32>, Vec<Box<dyn Primitive>>, usize);

impl StyleMaster {
    const FONT: &'static str = "font";
    const CLASS: &'static str = "class";
    const COULD_NOT_FIND_STYLE: &'static str = "Couldn't find style for";

    pub fn new(theme: Box<dyn Theme>) -> Result<StyleMaster, Box<dyn Error>> {
        let dyn_styles = theme.style();
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
                        Some(font_arc) => {
                            font_arc.clone()
                        }
                    };
                    map.insert(Self::FONT.to_string(), Property::Font(font_arc));
                } else {
                    panic!("Font properties should only be Str: {:?}", map);
                }
            }
            styles.push(map);
        }
        Ok(StyleMaster {
            _fonts: fonts,
            styles,
            theme,
        })
    }
    fn one_widget<T: Primitive + Debug>(size: Vector2D<f32>, vec_prim_rc: Vec<PrimitiveOrOneRef<T>>, reference: Rc<T>)
                                        -> Result<OneWidget, Box<dyn Error>> {
        debug!("{:?}", vec_prim_rc);
        let mut found = None;
        for (index, e) in vec_prim_rc.iter().enumerate() {
            if let PrimitiveOrOneRef::Ref(t) = e {
                if Rc::ptr_eq(t, &reference) {
                    found = Some(index);
                    break;
                }
            }
        }
        let index = found.ok_or("Couldn't find Ref")?;
        drop(reference);
        let vec_prim = vec_prim_rc.into_iter().map(|e| {
            match e {
                PrimitiveOrOneRef::Ref(r) => {
                    Box::new(Rc::try_unwrap(r).expect(
                        "This shouldn't fail, If so make sure the reference is dropped before."))
                }
                PrimitiveOrOneRef::Prim(p) => {
                    p
                }
            }
        }).collect();
        Ok((size, vec_prim, index))
    }
    pub fn one_button(&self, size: Vector2D<f32>, text: &str)
                      -> Result<OneWidget, Box<dyn Error>> {
        for style in &self.styles {
            if let Some(Property::Str(class)) = style.get(Self::CLASS) {
                if class == Button::class_name() {
                    let theme_style = ThemeButtonStyle::new(style.clone())?;
                    let (size, vec_prim_rc, text)
                        = self.theme.for_button(size, text, theme_style);
                    return Self::one_widget(size, vec_prim_rc, text);
                }
            }
        }
        debug!("{:?}", self.styles);
        Err(format!("{} {}", Self::COULD_NOT_FIND_STYLE, Button::class_name()).into())
    }
    pub fn one_textbox(&self, size: Vector2D<f32>, text: &str)
                       -> Result<OneWidget, Box<dyn Error>> {
        for style in &self.styles {
            if let Some(Property::Str(class)) = style.get(Self::CLASS) {
                if class == TextBox::class_name() {
                    let theme_style = ThemeTextBoxStyle::new(style.clone())?;
                    let (size, vec_prim_rc, text)
                        = self.theme.for_text_box(size, text, theme_style);
                    return Self::one_widget(size, vec_prim_rc, text);
                }
            }
        }
        debug!("{:?}", self.styles);
        Err(format!("{} {}", Self::COULD_NOT_FIND_STYLE, TextBox::class_name()).into())
    }
    pub fn one_image(&self, size: Vector2D<f32>, path: Box<Path>)
                     -> Result<OneWidget, Box<dyn Error>> {
        for style in &self.styles {
            if let Some(Property::Str(class)) = style.get(Self::CLASS) {
                if class == Image::class_name() {
                    let theme_style = ThemeImageStyle::new(style.clone())?;
                    let (size, vec_prim_rc, text)
                        = self.theme.for_image(size, path, theme_style)?;
                    return Self::one_widget(size, vec_prim_rc, text);
                }
            }
        }
        debug!("{:?}", self.styles);
        Err(format!("{} {}", Self::COULD_NOT_FIND_STYLE, Image::class_name()).into())
    }
}