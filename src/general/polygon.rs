use std::fmt::{Debug, Formatter};

use sdl2::sys;

use crate::general::{Color, Rect, Vector2D};

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
            Vector2D::new(rect.x() as f32, rect.y() as f32),
            color_for_all.clone(),
            Vector2D::new(0.0, 0.0),
        );
        let bottom_left = Self::new_vertex(
            Vector2D::new(rect.x() as f32, rect.bottom() as f32),
            color_for_all.clone(),
            Vector2D::new(0.0, 1.0),
        );
        let bottom_right = Self::new_vertex(
            Vector2D::new(rect.right() as f32, rect.bottom() as f32),
            color_for_all.clone(),
            Vector2D::new(1.0, 1.0),
        );
        let top_right = Self::new_vertex(
            Vector2D::new(rect.right() as f32, rect.y() as f32),
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
        Self::new_square_with_colors(
            size,
            radius,
            (color.clone(), color.clone(), color.clone(), color),
        )
    }

    pub fn new_square_with_colors(
        size: Vector2D<f32>,
        _radius: f32,
        colors: (Color, Color, Color, Color),
    ) -> Polygon {
        let top_left = Self::new_vertex(Vector2D::new(0.0, 0.0), colors.0, Default::default());
        let bottom_left =
            Self::new_vertex(Vector2D::new(0.0, size.y()), colors.1, Default::default());
        let bottom_right = Self::new_vertex(
            Vector2D::new(size.x(), size.y()),
            colors.2,
            Default::default(),
        );
        let top_right =
            Self::new_vertex(Vector2D::new(size.x(), 0.0), colors.3, Default::default());
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

        Polygon { vers, inds }
    }

    fn new_vertex(
        position: Vector2D<f32>,
        color: Color,
        tex_coord: Vector2D<f32>,
    ) -> sys::SDL_Vertex {
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
