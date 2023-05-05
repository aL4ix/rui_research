use std::rc::Rc;

use glyph_brush::ab_glyph::FontArc;

use crate::general::{Color, Vector2D};
use crate::widgets::Primitive;
use crate::widgets::primitives::Text;
use crate::widgets::themes::style::{Style, ThemeButtonStyle};

pub enum PrimitiveOrOneRef<T> {
    Ref(Rc<T>),
    Prim(Box<dyn Primitive>),
}

pub enum PrimitiveOrTwoRef<T1, T2> {
    Ref1(Rc<T1>),
    Ref2(Rc<T2>),
    Prim(Box<dyn Primitive>),
}

#[derive(Debug, Clone)]
pub enum Property {
    Float(f32),
    Font(FontArc),
    Str(String),
    Col(Color),
    Usize(usize),
    Vec2D(Vector2D<f32>),
}

impl TryInto<f32> for Property {
    type Error = String;

    fn try_into(self) -> Result<f32, Self::Error> {
        match self {
            Property::Float(v) => {
                Ok(v)
            }
            _ => {
                Err("Couldn't convert Property into ".to_string())
            }
        }
    }
}

impl TryInto<FontArc> for Property {
    type Error = String;

    fn try_into(self) -> Result<FontArc, Self::Error> {
        match self {
            Property::Font(v) => {
                Ok(v)
            }
            _ => {
                Err("Couldn't convert Property into ".to_string())
            }
        }
    }
}

impl TryInto<String> for Property {
    type Error = String;

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            Property::Str(v) => {
                Ok(v)
            }
            _ => {
                Err("Couldn't convert Property into ".to_string())
            }
        }
    }
}

impl TryInto<Color> for Property {
    type Error = String;

    fn try_into(self) -> Result<Color, Self::Error> {
        match self {
            Property::Col(v) => {
                Ok(v)
            }
            _ => {
                Err("Couldn't convert Property into ".to_string())
            }
        }
    }
}

impl TryInto<usize> for Property {
    type Error = String;

    fn try_into(self) -> Result<usize, Self::Error> {
        match self {
            Property::Usize(v) => {
                Ok(v)
            }
            _ => {
                Err("Couldn't convert Property into ".to_string())
            }
        }
    }
}

impl TryInto<Vector2D<f32>> for Property {
    type Error = String;

    fn try_into(self) -> Result<Vector2D<f32>, Self::Error> {
        match self {
            Property::Vec2D(v) => {
                Ok(v)
            }
            _ => {
                Err("Couldn't convert Property into ".to_string())
            }
        }
    }
}

pub trait Theme {
    fn for_button(&self, size: Vector2D<f32>, text: &str, style: ThemeButtonStyle) -> (Vector2D<f32>, Vec<PrimitiveOrOneRef<Text>>, Rc<Text>);
    fn for_textbox(&self, size: Vector2D<f32>, text: &str, font: FontArc, font_size: f32, color_text: Color,
                   color_background: Color) -> (Vector2D<f32>, Vec<PrimitiveOrOneRef<Text>>, Rc<Text>);
    fn style(&self) -> Vec<Box<dyn Style>>;
}