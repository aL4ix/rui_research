use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::sync::{Arc, Mutex};

use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::sys;
use sdl2::video::WindowContext;

use crate::sdl_engine::render_geometry;
use crate::tex_man::TextureManager;
use crate::texture::SoftTexture;

#[derive(Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Clone)]
pub struct Polygon {
    pub(crate) vers: Vec<sys::SDL_Vertex>,
    pub(crate) inds: Vec<i32>,
}

impl Debug for Polygon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let vers = self
            .vers
            .iter()
            .map(|v| {
                format!(
                    "SDL_Vertex {{ x: {}, y: {}, tx: {}, ty: {} }}",
                    v.position.x, v.position.y, v.tex_coord.x, v.tex_coord.y
                )
            })
            .collect::<Vec<String>>()
            .join(", ");
        let inds = self
            .inds
            .iter()
            .map(|i| format!("{}", i))
            .collect::<Vec<String>>()
            .join(" ,");
        write!(f, "Polygon {{ vers: [{}], inds: [{}] }}", vers, inds)
    }
}

impl Polygon {
    pub fn new_for_rect_texture(r: Rect, alpha: u8) -> Polygon {
        let alpha_for_all_vertices = alpha;
        let top_left = sys::SDL_Vertex {
            position: sys::SDL_FPoint {
                x: r.x as f32,
                y: r.y as f32,
            },
            color: sys::SDL_Color {
                r: 255,
                g: 255,
                b: 255,
                a: alpha_for_all_vertices,
            },
            tex_coord: sys::SDL_FPoint {
                x: 0.0,
                y: 0.0,
            },
        };
        let bottom_left = sys::SDL_Vertex {
            position: sys::SDL_FPoint {
                x: r.x as f32,
                y: r.bottom() as f32,
            },
            color: sys::SDL_Color {
                r: 255,
                g: 255,
                b: 255,
                a: alpha_for_all_vertices,
            },
            tex_coord: sys::SDL_FPoint {
                x: 0.0,
                y: 1.0,
            },
        };
        let bottom_right = sys::SDL_Vertex {
            position: sys::SDL_FPoint {
                x: r.right() as f32,
                y: r.bottom() as f32,
            },
            color: sys::SDL_Color {
                r: 255,
                g: 255,
                b: 255,
                a: alpha_for_all_vertices,
            },
            tex_coord: sys::SDL_FPoint {
                x: 1.0,
                y: 1.0,
            },
        };
        let top_right = sys::SDL_Vertex {
            position: sys::SDL_FPoint {
                x: r.right() as f32,
                y: r.y as f32,
            },
            color: sys::SDL_Color {
                r: 255,
                g: 255,
                b: 255,
                a: alpha_for_all_vertices,
            },
            tex_coord: sys::SDL_FPoint {
                x: 1.0,
                y: 0.0,
            },
        };
        // OpenGLs ordering standard for sending vertices is counter-clockwise, SDL2 is not
        // OpenGL-only but I guess D3D and Vulkan would be OK with this too.
        Polygon {
            vers: vec![top_left, bottom_left, bottom_right, top_right],
            inds: vec![0, 1, 2, 2, 3, 0],
        }
    }
}


/// A representation of SDL's geometry as defined in SDL_RenderGeometry
#[derive(Clone, Debug)]
pub struct TexturedPolygon {
    pub(crate) poly: Polygon,
    pub(crate) tex: Option<Arc<Mutex<dyn SoftTexture>>>,
}

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
}

#[derive(Debug)]
pub struct Size2D {
    pub(crate) width: u32,
    pub(crate) height: u32,
}
