use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use std::error::Error;

use crate::window::WindowSpecs;

pub trait Window {
    fn get_specs(&self) -> &WindowSpecs;
    fn event_key_down(&mut self, key: Keycode);
    fn event_mouse_button_down(&self, mouse_btn: MouseButton, x: i32, y: i32);
    fn build(&mut self) -> Result<(), Box<(dyn Error)>>;
    fn render(&mut self) -> Result<(), Box<(dyn Error)>>;
    fn clear_canvas(&mut self);
    fn present_canvas(&mut self);
}