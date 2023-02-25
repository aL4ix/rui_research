use std::thread::sleep;
use std::time::Duration;

use sdl2::init;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

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