use std::ptr;

use emscripten_main_loop::MainLoopEvent;
use emscripten_main_loop::MainLoopEvent::{Continue, Terminate};
use sdl2::{EventPump, init, sys, VideoSubsystem};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, Texture, WindowCanvas};

use crate::window::Window;

#[allow(dead_code)]
pub struct SDLEngine {
    sdl_context: sdl2::Sdl,
    sdl_video: VideoSubsystem,
    canvas: WindowCanvas,
    window: Window,
    event_pump: EventPump,
}

impl SDLEngine {
    pub fn new_main_loop(_windows_dsl: String) -> Result<(), Box<(dyn std::error::Error)>> {
        let sdl_context = init()?;
        let sdl_video = sdl_context.video()?;
        let sdl_window = sdl_video.window("Title", 800, 600).build()?;
        let canvas = sdl_window.into_canvas().build()?;

        let window = Window::new()?;

        let event_pump = sdl_context.event_pump()?;
        let sdl_engine = SDLEngine {
            sdl_context,
            sdl_video,
            canvas,
            window,
            event_pump,
        };

        emscripten_main_loop::run(sdl_engine);
        Ok(())
    }
}

impl emscripten_main_loop::MainLoop for SDLEngine {
    fn main_loop(&mut self) -> MainLoopEvent {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return Terminate,
                Event::KeyDown { keycode: Some(key), .. } => self.window.key_down(key),
                _ => {}
            }
        }
        self.window.build().expect("Build()");

        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        self.window.render(&mut self.canvas).expect("Render()");

        self.canvas.present();
        // sleep(Duration::new(0, 1_000_000_000u32 / 30));
        Continue
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
