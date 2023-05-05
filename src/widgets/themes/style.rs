use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
use std::rc::Rc;

use glyph_brush::ab_glyph::FontArc;
use log::info;

use crate::general::{Color, Vector2D};
use crate::utils::Assets;
use crate::widgets::Primitive;
use crate::widgets::themes::theme::{PrimitiveOrOneRef, Property, Theme};

type PropertiesMap = HashMap<String, Property>;

pub trait Style: Debug {
    fn to_properties_map(&self) -> PropertiesMap;
}

#[derive(Debug)]
pub struct ButtonStyle {
    pub to_id: usize,
    pub to_group: String,
    pub color: Vec<u8>,
    pub background_color: Vec<u8>,
    pub size: Vec<f32>,
    pub font: String,
    pub font_size: f32,
    pub extra: Vec<(String, Property)>,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        ButtonStyle {
            to_id: 0,
            to_group: "".to_string(),
            color: Default::default(),
            background_color: Default::default(),
            size: Default::default(),
            font: "".to_string(),
            font_size: 0.0,
            extra: Default::default(),
        }
    }
}

impl Style for ButtonStyle {
    fn to_properties_map(&self) -> PropertiesMap {
        use Property::*;
        let s = |s: &str| s.to_string();
        let mut r = HashMap::new();
        // TODO change to macro
        r.insert(s("class"), Str("Button".to_string()));
        r.insert(s("to_id"), Usize(self.to_id));
        r.insert(s("to_group"), Str(self.to_group.clone()));
        r.insert(s("color"), Col(Color::from(&self.color)));
        r.insert(s("background_color"), Col(Color::from(&self.background_color)));
        // r.insert(s("size"), Vec2D(Vector2D::from(&self.size)));
        // TODO how to handle properties like size that are optional?
        // Maybe not send anything, then in one_widget if it doesn't exist replace it with size
        // parameter from sizer
        r.insert(s("font"), Str(self.font.clone()));
        r.insert(s("font_size"), Float(self.font_size));
        r.extend(self.extra.clone());
        r
    }
}

#[derive(Debug)]
pub struct TextBoxStyle {
    pub to_id: usize,
    pub to_group: String,
    pub color: Vec<u8>,
    pub background_color: Vec<u8>,
    pub size: Vec<f32>,
    pub font: String,
    pub font_size: f32,
    pub extra: Vec<(String, Property)>,
}

impl Default for TextBoxStyle {
    fn default() -> Self {
        TextBoxStyle {
            to_id: 0,
            to_group: "".to_string(),
            color: Default::default(),
            background_color: Default::default(),
            size: Default::default(),
            font: "".to_string(),
            font_size: 0.0,
            extra: Default::default(),
        }
    }
}

impl Style for TextBoxStyle {
    fn to_properties_map(&self) -> PropertiesMap {
        use Property::*;
        let s = |s: &str| s.to_string();
        let mut r = HashMap::new();
        // TODO change to macro
        r.insert(s("class"), Str("TextBox".to_string()));
        r.insert(s("to_id"), Usize(self.to_id));
        r.insert(s("to_group"), Str(self.to_group.clone()));
        r.insert(s("color"), Col(Color::from(&self.color)));
        r.insert(s("background_color"), Col(Color::from(&self.background_color)));
        // r.insert(s("size"), Vec2D(Vector2D::from(&self.size)));
        r.insert(s("font"), Str(self.font.clone()));
        r.insert(s("font_size"), Float(self.font_size));
        r.extend(self.extra.clone());
        r
    }
}

pub struct ThemeButtonStyle {
    pub color: Color,
    pub background_color: Color,
    pub size: Vector2D<f32>,
    pub font: FontArc,
    pub font_size: f32,
    pub extra: HashMap<String, Property>,
}

impl ThemeButtonStyle {
    pub fn new(mut properties: PropertiesMap) -> Result<ThemeButtonStyle, Box<dyn Error>> {
        // TODO change to macro
        let color = properties.remove("color").ok_or("No color")?.try_into()?;
        let background_color = properties.remove("background_color").ok_or("No background_color")?.try_into()?;
        let size = properties.remove("size").ok_or("No size")?.try_into()?;
        let font = properties.remove("font").ok_or("No font")?.try_into()?;
        let font_size = properties.remove("font_size").ok_or("No font_size")?.try_into()?;
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

pub struct StyleMaster {
    fonts: HashMap<String, FontArc>,
    styles: Vec<PropertiesMap>,
    theme: Box<dyn Theme>,
}

impl StyleMaster {
    pub fn new(theme: Box<dyn Theme>) -> Result<StyleMaster, Box<dyn Error>> {
        let dyn_styles = theme.style();
        let mut fonts = HashMap::new();
        let mut styles = Vec::with_capacity(dyn_styles.len());
        for dyn_style in dyn_styles {
            info!("{:?}", dyn_style);
            let mut map = dyn_style.to_properties_map();
            let font_const_str = "font";
            if let Some(font_property) = map.get(font_const_str) {
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
                    map.insert(font_const_str.to_string(), Property::Font(font_arc));
                } else {
                    panic!("Font properties should only be Str: {:?}", map);
                }
            }
            styles.push(map);
        }
        Ok(StyleMaster {
            fonts,
            styles,
            theme,
        })
    }
    pub fn one_button(&self, size: Vector2D<f32>, text: &str)
                      -> Result<(Vector2D<f32>, Vec<Box<dyn Primitive>>, usize), Box<dyn Error>> {
        for style in &self.styles {
            if let Some(Property::Str(class)) = style.get("") {
                if class == "Button" {
                    let button_style = ThemeButtonStyle::new(style.clone())?;
                    let (size, vec_prim_rc, text)
                        = self.theme.for_button(size, text, button_style);
                    let id = text.id();
                    drop(text);
                    let vec_prim = vec_prim_rc.into_iter().map(|e| {
                        match e {
                            PrimitiveOrOneRef::Ref(r) => {
                                Box::new(Rc::try_unwrap(r).unwrap())
                            }
                            PrimitiveOrOneRef::Prim(p) => {
                                p
                            }
                        }
                    }).collect();
                    return Ok((size, vec_prim, id));
                }
            }
        }
        Err("Couldn't find style for Button".into())
    }
}