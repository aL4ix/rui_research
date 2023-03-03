use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::sync::{Arc, Mutex};

use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::sys;

use crate::sdl_engine::render_geometry;

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
        write!(f, "SDLPolygon {{ vers: [{}], inds: [{}] }}", vers, inds)
    }
}

impl Polygon {
    pub fn new() -> Polygon {
        Polygon {
            vers: vec![],
            inds: vec![],
        }
    }
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
#[derive(Clone)]
pub struct TexturedPolygon {
    pub(crate) poly: Polygon,
    pub(crate) tex: Option<Arc<Mutex<Texture>>>,
}

impl Debug for TexturedPolygon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let poly = format!("{:?}", self.poly);
        write!(f, "SDLPolygon {{ {} }}", poly)
    }
}

/// It's a group of multiple polygons
#[derive(Debug, Clone)]
pub struct Body {
    #[allow(dead_code)]
    pub(crate) class: String,
    pub(crate) polygons: Vec<TexturedPolygon>,
}

impl Body {
    pub fn render(&self, canvas: &mut WindowCanvas) -> Result<(), Box<(dyn std::error::Error)>> {
        for tex_poly in &self.polygons {
            if let Some(t) = &tex_poly.tex {
                let guard = t.lock().unwrap();
                let tex = Some(guard.deref());
                render_geometry(canvas, tex, &tex_poly.poly.vers, &tex_poly.poly.inds)?;
            } else {
                // Is there a way to avoid duplicating render_geometry? the tex mutex guard is blocking it
                render_geometry(canvas, None, &tex_poly.poly.vers, &tex_poly.poly.inds)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Size2D {
    pub(crate) width: u32,
    pub(crate) height: u32,
}
