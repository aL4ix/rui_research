use std::sync::{Arc, Mutex};

use rui_research::engines::sdl::SDLEngine;
use rui_research::general::Vector2D;
use rui_research::widgets::{Primitive, TextBox};
use rui_research::widgets::themes::{SimpleTheme, StyleMaster};
use rui_research::window::WindowBuilder;

fn main() -> Result<(), Box<(dyn std::error::Error)>> {
    let mut sdl_engine = SDLEngine::init()?;
    let style = StyleMaster::new(Box::new(SimpleTheme))?;
    let mut window_builder = WindowBuilder::new()?;
    let string = "Phrase 1
Phrase 2
Phrase 3";
    let mut i = 0;
    for line in string.lines() {
        let mut text2 = TextBox::new(0, line, &style)?;
        text2.set_position(Vector2D::new(0., 50. * i as f32));
        window_builder.add_widget(i, Arc::new(Mutex::new(text2)), 1);
        i += 1;
    }

    sdl_engine.add_window_builder(window_builder)?;
    sdl_engine.main_loop();
    Ok(())
}