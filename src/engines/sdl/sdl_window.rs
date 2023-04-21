use std::error::Error;

use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::render::Canvas;

use crate::window::{Window, WindowSpecs};

pub struct SDLWindow {
    window_specs: WindowSpecs,
    canvas: Canvas<sdl2::video::Window>,
}

impl SDLWindow {
    pub fn new(window_specs: WindowSpecs, sdl_window: sdl2::video::Window)
               -> Result<SDLWindow, Box<dyn Error>> {
        let canvas = sdl_window.into_canvas().build()?;
        Ok(SDLWindow {
            window_specs,
            canvas,
        })
    }
}

impl Window for SDLWindow {
    fn get_specs(&self) -> &WindowSpecs {
        &self.window_specs
    }

    fn event_key_down(&mut self, key: Keycode) {
        self.window_specs.event_key_down(key)
    }

    fn event_mouse_button_down(&self, mouse_btn: MouseButton, x: i32, y: i32) {
        self.window_specs.event_mouse_button_down(mouse_btn, x, y)
    }

    fn build(&mut self) -> Result<(), Box<(dyn Error)>> {
        self.window_specs.build()
    }

    fn render(&mut self) -> Result<(), Box<(dyn Error)>> {
        self.window_specs.render(&mut self.canvas)
    }

    fn clear_canvas(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    fn present_canvas(&mut self) {
        self.canvas.present();
    }
}