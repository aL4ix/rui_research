use std::sync::{Arc, Mutex};

use crate::general::Polygon;
use crate::texture::SoftTexture;

/// A representation of SDL's geometry as defined in SDL_RenderGeometry
#[derive(Clone, Debug)]
pub struct TexturedPolygon {
    pub(crate) poly: Polygon,
    pub(crate) tex: Option<Arc<Mutex<dyn SoftTexture>>>,
}