use std::collections::HashMap;
use std::default::Default;
use std::error::Error;
use std::ptr;

use emscripten_main_loop::MainLoopEvent;
use emscripten_main_loop::MainLoopEvent::{Continue, Terminate};
use log::debug;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, Texture};
use sdl2::{init, sys, EventPump, VideoSubsystem};

pub use sdl_window::SDLWindow;

use crate::window::{Window, WindowBuilder};

mod sdl_window;

#[allow(dead_code)]
pub struct SDLEngine {
    sdl_context: sdl2::Sdl,
    sdl_video: VideoSubsystem,
    windows: HashMap<u32, SDLWindow>,
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
    pub fn init() -> Result<SDLEngine, Box<(dyn Error)>> {
        // sdl2::hint::set("SDL_TOUCH_MOUSE_EVENTS", "1"); // Maybe useful for android
        let sdl_context = init()?;
        debug!("RUI Started SDL");
        let sdl_video = sdl_context.video()?;
        debug!("RUI Started video");

        let event_pump = sdl_context.event_pump()?;
        let sdl_engine = SDLEngine {
            sdl_context,
            sdl_video,
            windows: Default::default(),
            event_pump,
            user_event_handler: None,
        };
        Ok(sdl_engine)
    }
    pub fn add_window_builder(
        &mut self,
        window_specs: WindowBuilder,
    ) -> Result<(), Box<dyn Error>> {
        let sdl_window = self.sdl_video.window("Title1", window_specs.width(),
                                               window_specs.height()).build()?;
        let id = sdl_window.id();
        debug!("Created window {}", id);
        self.windows
            .insert(id, SDLWindow::new(window_specs, sdl_window)?);
        Ok(())
    }
    #[allow(dead_code)]
    pub fn set_user_event_handler(
        &mut self,
        user_event_handler: Option<fn(&Event) -> MainLoopStatus>,
    ) {
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
                Event::KeyDown {
                    window_id,
                    keycode: Some(key),
                    ..
                } => self
                    .windows
                    .get_mut(&window_id)
                    .expect("")
                    .event_key_down(key),
                Event::MouseButtonDown {
                    window_id,
                    mouse_btn,
                    x,
                    y,
                    ..
                } => self
                    .windows
                    .get_mut(&window_id)
                    .expect("")
                    .event_mouse_button_down(mouse_btn, x, y),
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
        for window in &mut self.windows.values_mut() {
            window.build_geometry().expect("Build()");
        }

        for window in &mut self.windows.values_mut() {
            window.clear_canvas();
            window.render().expect("Render()");
            window.present_canvas();
        }

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
            return Err(format!(
                "Failed at SDL_RenderGeometry {}",
                sdl2::get_error()
            ));
        }
    }

    Ok(())
}
