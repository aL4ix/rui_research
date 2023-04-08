use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::sync::{Arc, Mutex};

use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::sys;
use sdl2::video::WindowContext;

use crate::sdl_engine::render_geometry;
use crate::tex_man::TextureManager;
use crate::texture::SoftTexture;

/// Alpha of 255 is opaque, 0 is transparent
#[derive(Clone, Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    pub fn r(&self) -> u8 {
        self.r
    }
    pub fn g(&self) -> u8 {
        self.g
    }
    pub fn b(&self) -> u8 {
        self.b
    }
    pub fn a(&self) -> u8 {
        self.a
    }
}

#[derive(Clone, Debug)]
pub struct Vector2D<T> {
    x: T,
    y: T,
}

impl<T: Copy + Default> Vector2D<T> {
    pub fn new(x: T, y: T) -> Vector2D<T> {
        Vector2D {
            x,
            y,
        }
    }
    pub fn x(&self) -> T {
        self.x
    }
    pub fn y(&self) -> T {
        self.y
    }
    pub fn unpack(&self) -> (T, T) {
        (self.x, self.y)
    }
}

impl<T: Copy + Default> Default for Vector2D<T> {
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}

#[derive(Clone, Debug, Copy)]
pub struct Rect<T> {
    x: T,
    y: T,
    width: T,
    height: T,
}

impl<T: std::ops::Add<Output=T> + Copy + Default> Rect<T> {
    pub fn new(x: T, y: T, width: T, height: T) -> Rect<T> {
        Rect {
            x,
            y,
            width,
            height,
        }
    }
    pub fn new_zero() -> Rect<T> {
        Self::new(Default::default(), Default::default(), Default::default(),
                  Default::default())
    }
    pub fn bottom(&self) -> T {
        self.y + self.height
    }
    pub fn right(&self) -> T {
        self.x + self.width
    }
    pub fn x(&self) -> T {
        self.x
    }
    pub fn y(&self) -> T {
        self.y
    }
    pub fn width(&self) -> T {
        self.width
    }
    pub fn height(&self) -> T {
        self.height
    }
    pub fn unpack(&self) -> (T, T, T, T) {
        (self.x, self.y, self.width, self.height)
    }
}

impl Into<Option<sdl2::rect::Rect>> for Rect<u32> {
    fn into(self) -> Option<sdl2::rect::Rect> {
        Some(sdl2::rect::Rect::new(self.x().try_into().unwrap(),
                                   self.y().try_into().unwrap(),
                                   self.width(),
                                   self.height()))
    }
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
    pub fn new_rect_for_texture(rect: Rect<u32>, alpha_for_all_vertices: u8) -> Polygon {
        let color_for_all = Color::new(255, 255, 255, alpha_for_all_vertices);
        let top_left = Self::new_vertex(
            Vector2D::new(rect.x as f32, rect.y as f32),
            color_for_all.clone(),
            Vector2D::new(0.0, 0.0),
        );
        let bottom_left = Self::new_vertex(
            Vector2D::new(rect.x as f32, rect.bottom() as f32),
            color_for_all.clone(),
            Vector2D::new(0.0, 1.0),
        );
        let bottom_right = Self::new_vertex(
            Vector2D::new(rect.right() as f32, rect.bottom() as f32),
            color_for_all.clone(),
            Vector2D::new(1.0, 1.0),
        );
        let top_right = Self::new_vertex(
            Vector2D::new(rect.right() as f32, rect.y as f32),
            color_for_all,
            Vector2D::new(1.0, 0.0),
        );
        // OpenGLs ordering standard for sending vertices is counter-clockwise, SDL2 is not
        // OpenGL-only but I guess D3D and Vulkan would be OK with this too.
        Polygon {
            vers: vec![top_left, bottom_left, bottom_right, top_right],
            inds: vec![0, 1, 2, 2, 3, 0],
        }
    }

    pub fn new_square(size: Vector2D<f32>, radius: f32, color: Color) -> Polygon {
        Self::new_square_with_colors(size, radius, (color.clone(), color.clone(),
                                                    color.clone(), color))
    }

    pub fn new_square_with_colors(size: Vector2D<f32>, _radius: f32,
                                  colors: (Color, Color, Color, Color)) -> Polygon {
        let top_left = Self::new_vertex(
            Vector2D::new(0.0, 0.0),
            colors.0,
            Default::default(),
        );
        let bottom_left = Self::new_vertex(
            Vector2D::new(0.0, size.y()),
            colors.1,
            Default::default(),
        );
        let bottom_right = Self::new_vertex(
            Vector2D::new(size.x(), size.y()),
            colors.2,
            Default::default(),
        );
        let top_right = Self::new_vertex(
            Vector2D::new(size.x(), 0.0),
            colors.3,
            Default::default(),
        );
        Polygon {
            vers: vec![top_left, bottom_left, bottom_right, top_right],
            inds: vec![0, 1, 2, 2, 3, 0],
        }
    }

    pub fn new_reg_poly(size: Vector2D<f32>, sides: u32, rotate: f32) -> Polygon {
        let max = 2.0 * std::f32::consts::PI;
        let step = max / sides as f32;
        let (w, h) = size.unpack();
        let x = w / 2.0;
        let y = h / 2.0;
        let mut vers = Vec::with_capacity(sides as usize);
        for i in 0..sides {
            let cur_step = step * i as f32 + rotate;
            let position = Vector2D::new(x + w * cur_step.sin(), y + h * -cur_step.cos());
            let color = Color::new(255, 255, 255, 0);
            let tex_coords: Vector2D<f32> = Default::default();
            vers.push(Self::new_vertex(position, color, tex_coords));
        }
        // vers.get_mut(0).expect("").color = sys::SDL_Color {
        //     r: 128,
        //     g: 128,
        //     b: 128,
        //     a: 0,
        // };

        let mut inds = Vec::with_capacity(((sides - 2) * 3) as usize);
        inds.push(0);
        inds.push(1);
        inds.push(2);
        for i in 0..sides as i32 - 3 {
            let start = 2 + i;
            inds.push(0);
            inds.push(start);
            inds.push(start + 1);
        }

        Polygon {
            vers,
            inds,
        }
    }

    fn new_vertex(position: Vector2D<f32>, color: Color, tex_coord: Vector2D<f32>) -> sys::SDL_Vertex {
        sys::SDL_Vertex {
            position: sys::SDL_FPoint {
                x: position.x(),
                y: position.y(),
            },
            color: sys::SDL_Color {
                r: color.r(),
                g: color.g(),
                b: color.b(),
                a: color.a(),
            },
            tex_coord: sys::SDL_FPoint {
                x: tex_coord.x(),
                y: tex_coord.y(),
            },
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
    pub fn translate(&mut self, position: &Vector2D<f32>) {
        for tex_poly in &mut self.polygons {
            for mut ver in &mut tex_poly.poly.vers {
                ver.position.x += position.x;
                ver.position.y += position.y;
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

#[derive(Debug)]
pub struct Size2D {
    pub(crate) width: u32,
    pub(crate) height: u32,
}
