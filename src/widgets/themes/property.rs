use crate::general::{Color, Vector2D};
use crate::widgets::{ExtraStyle, ExtraStyleMap};
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
    ExtraProperties(ExtraStyleMap),
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

impl From<ExtraStyle> for Property {
    fn from(value: ExtraStyle) -> Self {
        Property::ExtraProperties(value.into_iter().collect())
    }
}

impl TryInto<f32> for Property {
    type Error = String;

    fn try_into(self) -> Result<f32, Self::Error> {
        match self {
            Property::Float(value) => Ok(value),
            _ => Err("Couldn't convert Property into Float".to_string()),
        }
    }
}

impl TryInto<FontArc> for Property {
    type Error = String;

    fn try_into(self) -> Result<FontArc, Self::Error> {
        match self {
            Property::Font(value) => Ok(value),
            _ => Err("Couldn't convert Property to Font".to_string()),
        }
    }
}

impl TryInto<String> for Property {
    type Error = String;

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            Property::Str(value) => Ok(value),
            _ => Err("Couldn't convert Property to Str".to_string()),
        }
    }
}

impl TryInto<Color> for Property {
    type Error = String;

    fn try_into(self) -> Result<Color, Self::Error> {
        match self {
            Property::Col(value) => Ok(value),
            _ => Err("Couldn't convert Property into Col".to_string()),
        }
    }
}

impl TryInto<usize> for Property {
    type Error = String;

    fn try_into(self) -> Result<usize, Self::Error> {
        match self {
            Property::Usize(value) => Ok(value),
            _ => Err("Couldn't convert Property into Usize".to_string()),
        }
    }
}

impl TryInto<Vector2D<f32>> for Property {
    type Error = String;

    fn try_into(self) -> Result<Vector2D<f32>, Self::Error> {
        match self {
            Property::Vec2D(value) => Ok(value),
            _ => Err("Couldn't convert Property into Vec2D".to_string()),
        }
    }
}

impl TryInto<ApplyTo> for Property {
    type Error = String;

    fn try_into(self) -> Result<ApplyTo, Self::Error> {
        match self {
            Property::ApplyTo(value) => Ok(value),
            _ => Err("Couldn't convert Property into ApplyTo".to_string()),
        }
    }
}

impl TryInto<Option<Vector2D<f32>>> for Property {
    type Error = String;

    fn try_into(self) -> Result<Option<Vector2D<f32>>, Self::Error> {
        match self {
            Property::None => Ok(None),
            Property::Vec2D(value) => Ok(Some(value)),
            _ => Err("Couldn't convert Property into Option<Vec2D>".to_string()),
        }
    }
}

impl TryInto<ExtraStyleMap> for Property {
    type Error = String;

    fn try_into(self) -> Result<ExtraStyleMap, Self::Error> {
        match self {
            Property::ExtraProperties(value) => Ok(value),
            _ => Err("Couldn't convert Property into ExtraStyleMap".to_string()),
        }
    }
}
