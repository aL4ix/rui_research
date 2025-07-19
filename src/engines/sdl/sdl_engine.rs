use std::collections::HashMap;
use std::default::Default;
use std::error::Error;

use emscripten_main_loop::MainLoopEvent;
use emscripten_main_loop::MainLoopEvent::{Continue, Terminate};
use log::debug;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::{init, EventPump, VideoSubsystem};

use crate::engines::sdl::SDLWindow;
use crate::window::{Window, WindowBuilder};

#[derive(PartialEq, Eq)]
pub enum MainLoopStatus {
    Continue,
    Terminate,
    Supress,
}

#[allow(dead_code)]
pub struct SDLEngine {
    sdl_context: sdl2::Sdl,
    sdl_video: VideoSubsystem,
    windows: HashMap<u32, SDLWindow>,
    event_pump: EventPump,
    user_event_handler: Option<fn(&Event) -> MainLoopStatus>,
}

impl SDLEngine {
    pub fn init() -> Result<SDLEngine, Box<(dyn Error)>> {
        // sdl2::hint::set("SDL_TOUCH_MOUSE_EVENTS", "1"); // Maybe useful for android
        let sdl_context: sdl2::Sdl = init()?;
        debug!("Rusty-UI Started SDL");
        let sdl_video = sdl_context.video()?;
        debug!("Rusty-UI Started video");

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
        let sdl_window = self
            .sdl_video
            .window("Rusty-UI", window_specs.width(), window_specs.height())
            .build()?;
        let id = sdl_window.id();
        debug!("Created window {}", id);
        self.windows
            .insert(id, SDLWindow::new(window_specs, sdl_window)?);
        Ok(())
    }
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
