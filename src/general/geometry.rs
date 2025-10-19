use std::ops::Deref;
use std::sync::{Arc, Mutex};

use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

use crate::general::{Polygon, TexturedPolygon, Vector2D};
use crate::texture::SoftTexture;
use crate::texture::TextureManager;
use crate::themes::PrimitivesManagerForThemes;

/// It's a group of multiple polygons
#[derive(Debug, Clone)]
pub struct Geometry {
    pub(crate) _class: String,
    pub(crate) polygons: Vec<TexturedPolygon>,
}

impl Geometry {
    pub fn new_empty_with_class(class: &str) -> Geometry {
        Geometry {
            _class: class.to_string(),
            polygons: vec![],
        }
    }
    pub fn new_for_texture(
        class: &str,
        tex: Arc<Mutex<dyn SoftTexture>>,
        poly: Polygon,
    ) -> Geometry {
        Geometry {
            _class: class.to_string(),
            polygons: vec![TexturedPolygon {
                poly,
                tex: Some(tex),
            }],
        }
    }
    pub fn new_from_geometries(class: &str, geometries: Vec<Geometry>) -> Geometry {
        let mut result = Self::new_empty_with_class(class);
        for mut geometry in geometries {
            result.polygons.append(&mut geometry.polygons);
        }
        result
    }
    pub fn new_from_prim_man(class: &str, prim_man: &mut PrimitivesManagerForThemes) -> Geometry {
        let mut geometries = Vec::with_capacity(prim_man.len());
        for (_, primitive) in prim_man.iter_mut() {
            if primitive.needs_update() {
                primitive.update_geometry();
            }
            geometries.push(primitive.clone_geometry());
        }
        Geometry::new_from_geometries(class, geometries)
    }
    pub fn render(
        &mut self,
        canvas: &mut WindowCanvas,
        tex_creator: &TextureCreator<WindowContext>,
        tex_man: &mut TextureManager,
    ) -> Result<(), Box<(dyn std::error::Error)>> {
        for tex_poly in &mut self.polygons {
            // info!("{:?}", tex_poly);
            if let Some(arc_tex) = &mut tex_poly.tex {
                let mut guard = arc_tex.lock().expect("geometry:Geometry:render");
                // info!("{:?}", guard);
                let rendered_tex = guard.render(tex_creator, tex_man)?;
                let guard = rendered_tex.borrow();
                let tex = Some(guard.deref());
                canvas.render_geometry(&tex_poly.poly.vers, tex, &tex_poly.poly.inds)?;
            } else {
                // Is there a way to avoid duplicating render_geometry? the tex mutex guard is blocking it
                canvas.render_geometry(&tex_poly.poly.vers, None, &tex_poly.poly.inds)?;
            }
        }
        Ok(())
    }
    pub fn translate(&mut self, position: &Vector2D<f32>) {
        for tex_poly in &mut self.polygons {
            for ver in &mut tex_poly.poly.vers {
                ver.position.x += position.x();
                ver.position.y += position.y();
            }
        }
    }
}

impl Default for Geometry {
    fn default() -> Self {
        Geometry {
            _class: "".to_string(),
            polygons: vec![],
        }
    }
}
