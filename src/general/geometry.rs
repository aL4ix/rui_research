use std::ops::Deref;
use std::sync::{Arc, Mutex};

use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

use crate::general::{Polygon, TexturedPolygon, Vector2D};
use crate::sdl_engine::render_geometry;
use crate::texture::TextureManager;
use crate::texture::SoftTexture;

/// It's a group of multiple polygons
#[derive(Debug, Clone)]
pub struct Geometry {
    #[allow(dead_code)]
    pub(crate) class: String,
    pub(crate) polygons: Vec<TexturedPolygon>,
}

impl Geometry {
    pub fn render(&mut self, canvas: &mut WindowCanvas, tex_creator: &TextureCreator<WindowContext>,
                  tex_man: &mut TextureManager) -> Result<(), Box<(dyn std::error::Error)>> {
        for tex_poly in &mut self.polygons {
            // info!("{:?}", tex_poly);
            if let Some(arc_tex) = &mut tex_poly.tex {
                let mut guard = arc_tex.lock().unwrap();
                // info!("{:?}", guard);
                let rendered_tex = guard.render(tex_creator, tex_man)?;
                let guard = rendered_tex.borrow();
                let tex = Some(guard.deref());
                render_geometry(canvas, tex, &tex_poly.poly.vers, &tex_poly.poly.inds)?;
            } else {
                // Is there a way to avoid duplicating render_geometry? the tex mutex guard is blocking it
                render_geometry(canvas, None, &tex_poly.poly.vers, &tex_poly.poly.inds)?;
            }
        }
        Ok(())
    }
    pub fn new_for_texture(class: &str, tex: Arc<Mutex<dyn SoftTexture>>, poly: Polygon) -> Geometry {
        Geometry {
            class: class.to_string(),
            polygons: vec![TexturedPolygon { poly, tex: Some(tex) }],
        }
    }
    pub fn translate(&mut self, position: &Vector2D<f32>) {
        for tex_poly in &mut self.polygons {
            for mut ver in &mut tex_poly.poly.vers {
                ver.position.x += position.x();
                ver.position.y += position.y();
            }
        }
    }
}

impl Default for Geometry {
    fn default() -> Self {
        Geometry {
            class: "".to_string(),
            polygons: vec![],
        }
    }
}