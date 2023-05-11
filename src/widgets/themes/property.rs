use crate::general::{Color, Vector2D};
use crate::widgets::themes::style::ApplyTo;
use glyph_brush::ab_glyph::FontArc;

#[derive(Debug, Clone)]
pub enum Property {
    None,
    Float(f32),
    Font(FontArc),
    Str(String),
    Col(Color),
    Usize(usize),
    Vec2D(Vector2D<f32>),
    ApplyTo(ApplyTo),
}

impl From<f32> for Property {
    fn from(value: f32) -> Self {
        Property::Float(value)
    }
}

impl From<FontArc> for Property {
    fn from(value: FontArc) -> Self {
        Property::Font(value)
    }
}

impl From<String> for Property {
    fn from(value: String) -> Self {
        Property::Str(value)
    }
}

impl From<Color> for Property {
    fn from(value: Color) -> Self {
        Property::Col(value)
    }
}

impl From<usize> for Property {
    fn from(value: usize) -> Self {
        Property::Usize(value)
    }
}

impl From<Vector2D<f32>> for Property {
    fn from(value: Vector2D<f32>) -> Self {
        Property::Vec2D(value)
    }
}

impl From<&Option<(f32, f32)>> for Property {
    fn from(value: &Option<(f32, f32)>) -> Self {
        match value {
            None => Property::None,
            Some(vec2d) => Property::Vec2D(Vector2D::new(vec2d.0, vec2d.1)),
        }
    }
}

impl From<ApplyTo> for Property {
    fn from(value: ApplyTo) -> Self {
        Property::ApplyTo(value)
    }
}

impl TryInto<f32> for Property {
    type Error = String;

    fn try_into(self) -> Result<f32, Self::Error> {
        match self {
            Property::Float(value) => Ok(value),
            _ => Err("Couldn't convert Property".to_string()),
        }
    }
}

impl TryInto<FontArc> for Property {
    type Error = String;

    fn try_into(self) -> Result<FontArc, Self::Error> {
        match self {
            Property::Font(value) => Ok(value),
            _ => Err("Couldn't convert Property".to_string()),
        }
    }
}

impl TryInto<String> for Property {
    type Error = String;

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            Property::Str(value) => Ok(value),
            _ => Err("Couldn't convert Property".to_string()),
        }
    }
}

impl TryInto<Color> for Property {
    type Error = String;

    fn try_into(self) -> Result<Color, Self::Error> {
        match self {
            Property::Col(value) => Ok(value),
            _ => Err("Couldn't convert Property".to_string()),
        }
    }
}

impl TryInto<usize> for Property {
    type Error = String;

    fn try_into(self) -> Result<usize, Self::Error> {
        match self {
            Property::Usize(value) => Ok(value),
            _ => Err("Couldn't convert Property".to_string()),
        }
    }
}

impl TryInto<Vector2D<f32>> for Property {
    type Error = String;

    fn try_into(self) -> Result<Vector2D<f32>, Self::Error> {
        match self {
            Property::Vec2D(value) => Ok(value),
            _ => Err("Couldn't convert Property".to_string()),
        }
    }
}

impl TryInto<ApplyTo> for Property {
    type Error = String;

    fn try_into(self) -> Result<ApplyTo, Self::Error> {
        match self {
            Property::ApplyTo(value) => Ok(value),
            _ => Err("Couldn't convert Property".to_string()),
        }
    }
}

impl TryInto<Option<Vector2D<f32>>> for Property {
    type Error = String;

    fn try_into(self) -> Result<Option<Vector2D<f32>>, Self::Error> {
        match self {
            Property::None => Ok(None),
            Property::Vec2D(value) => Ok(Some(value)),
            _ => Err("Couldn't convert Property".to_string()),
        }
    }
}
