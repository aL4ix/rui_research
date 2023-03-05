use std::ptr;
use std::thread::sleep;
use std::time::Duration;

use sdl2::{init, sys};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, Texture};

use crate::window::WindowBuilder;

pub struct SDLEngine {}

impl SDLEngine {
    pub fn main_loop(_windows_dsl: String) -> Result<(), Box<(dyn std::error::Error)>> {
        let sdl_context = init()?;
        let sdl_video = sdl_context.video()?;
        let window = sdl_video.window("Title", 800, 600).build()?;
        let mut canvas = window.into_canvas().build()?;

        let mut window_builder = WindowBuilder::new()?;

        let mut event_pump = sdl_context.event_pump()?;
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyDown {keycode: Some(key), ..} => window_builder.key_down(key),
                    _ => {}
                }
            }

            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();

            window_builder.render(&mut canvas)?;

            canvas.present();
            sleep(Duration::new(0, 1_000_000_000u32 / 30));
        }
        Ok(())
    }
}

pub fn render_geometry<C: sdl2::render::RenderTarget>(
    canvas: &mut Canvas<C>,
    texture: Option<&Texture>,
    vertices: &Vec<sys::SDL_Vertex>,
    indices: &Vec<i32>,
) -> Result<(), String> {
    if !vertices.is_empty() {
        let sdl_renderer = canvas.raw();
        let vers_num = vertices.len() as i32;
        let vers_ptr = (&vertices[0]) as *const sys::SDL_Vertex;
        let tex_ptr: *mut sys::SDL_Texture = match texture {
            None => ptr::null_mut(),
            Some(t) => t.raw(),
        };
        let ind_num = indices.len() as i32;
        let inds_ptr = match ind_num {
            0 => ptr::null(),
            _ => &indices[0],
        };
        let ret = unsafe {
            sys::SDL_RenderGeometry(sdl_renderer, tex_ptr, vers_ptr, vers_num, inds_ptr, ind_num)
        };
        if ret == -1 {
            return Err(format!("Failed at SDL_RenderGeometry {}", sdl2::get_error()));
        }
    }

    Ok(())
}
