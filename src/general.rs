use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};
use sdl2::rect::Rect;
use sdl2::sys;
use crate::texture::SoftTexture;

#[derive(Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Clone)]
pub struct SDLPolygon {
    pub vers: Vec<sys::SDL_Vertex>,
    pub inds: Vec<i32>,
}

impl Debug for SDLPolygon {
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

impl SDLPolygon {
    pub fn new_empty() -> SDLPolygon {
        SDLPolygon {
            vers: vec![],
            inds: vec![]
        }
    }

    pub fn new_for_rect_texture(r: Rect) -> SDLPolygon {
        let alpha_for_all_vertices = 255;
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
        // OpenGLs ordering standard for sending vertexes is counter-clockwise, SDL2 is not
        // OpenGL-only but I guess D3D and Vulkan would be OK with this too.
        SDLPolygon {
            vers: vec![top_left, bottom_left, bottom_right, top_right],
            inds: vec![0, 1, 2, 2, 3, 0],
        }
    }
}


/// A representation of SDL's geometry as defined in SDL_RenderGeometry
///
/// # Examples
///
/// ```
///
#[derive(Clone)]
pub struct SDLTexturedPolygon {
    pub poly: SDLPolygon,
    pub tex: Option<Arc<Mutex<dyn SoftTexture>>>,
    // TODO change to sdl2::render::Texture
    // What if i create an SDLTexture trait, then it could be a normal texture or a lazy,
    // and just call sdl_texture.get() and the normal texture returns a sdl2::texture, while a lazy
    // one process the lazy_tex and then returns it. but who is gonna own the lazy tex?
    // SDL_Tex?
}

impl Debug for SDLTexturedPolygon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let poly = format!("{:?}", self.poly);
        write!(f, "SDLPolygon {{ {} }}", poly)
    }
}

/// It's a group of separate polygons
#[derive(Debug, Clone)]
pub struct SDLBody {
    // TODO remove pub
    pub _name: String,
    // TODO remove pub
    pub polygons: Vec<SDLTexturedPolygon>,
}

#[derive(Debug)]
pub struct Size2D {
    pub(crate) width: u32,
    pub(crate) height: u32,
}