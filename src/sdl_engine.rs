use std::ptr;

use emscripten_main_loop::MainLoopEvent;
use emscripten_main_loop::MainLoopEvent::{Continue, Terminate};
use log::debug;
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
    windows: Vec<Window>,
    event_pump: EventPump,
    user_event_handler: Option<fn(&Event) -> MainLoopStatus>,
}

#[allow(dead_code)]
#[derive(PartialEq, Eq)]
pub enum MainLoopStatus {
    Continue,
    Terminate,
    Supress,
}

impl SDLEngine {
    pub fn init() -> Result<SDLEngine, Box<(dyn std::error::Error)>> {
        let sdl_context = init()?;
        debug!("RUI Started SDL");
        let sdl_video = sdl_context.video()?;
        let sdl_window = sdl_video.window("Title", 800, 600).build()?;
        debug!("RUI Started video");
        let canvas = sdl_window.into_canvas().build()?;

        let event_pump = sdl_context.event_pump()?;
        let sdl_engine = SDLEngine {
            sdl_context,
            sdl_video,
            canvas,
            windows: vec![],
            event_pump,
            user_event_handler: None,
        };
        Ok(sdl_engine)
    }
    pub fn add_window(&mut self, window: Window) {
        self.windows.push(window);
    }
    #[allow(dead_code)]
    pub fn set_user_event_handler(&mut self, user_event_handler: Option<fn(&Event) -> MainLoopStatus>) {
        self.user_event_handler = user_event_handler;
    }
    pub fn main_loop(self) {
        emscripten_main_loop::run(self);
    }
    pub fn process_events(&mut self) -> MainLoopStatus {
        for event in self.event_pump.poll_iter() {
            if let Some(event_handler) = self.user_event_handler {
                match event_handler(&event) {
                    MainLoopStatus::Terminate => return MainLoopStatus::Terminate,
                    MainLoopStatus::Supress => continue,
                    MainLoopStatus::Continue => {}
                }
            }
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return MainLoopStatus::Terminate,
                // TODO fix getting hardcoded window
                Event::KeyDown { keycode: Some(key), .. } => self.windows[0].event_key_down(key),
                Event::MouseButtonDown { timestamp: _timestamp, window_id: _window_id, which: _which,
                    mouse_btn, clicks: _clicks, x, y } =>
                    self.windows[0].event_mouse_button_down(mouse_btn, x, y),
                _ => {}
            }
        }
        MainLoopStatus::Continue
    }
}

impl emscripten_main_loop::MainLoop for SDLEngine {
    fn main_loop(&mut self) -> MainLoopEvent {
        if self.process_events() == MainLoopStatus::Terminate {
            return Terminate;
        }
        for window in &mut self.windows {
            window.build().expect("Build()");
        }

        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        for window in &mut self.windows {
            window.render(&mut self.canvas).expect("Render()");
        }

        self.canvas.present();
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
