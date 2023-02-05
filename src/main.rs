use std::path::Path;
use std::sync::{Arc, mpsc};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::init;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use crate::texture::*;

mod texture;


fn main() -> Result<(), String> {
    println!("Hello, world!");
    let sdl_context = init()?;
    let sdl_video = sdl_context.video()?;
    let window = sdl_video.window("Title", 800, 600).build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let tex_creator = Arc::new(canvas.texture_creator());

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let nct = NeverChangingTexture::new(Box::from(Path::new("image.bmp")));
        tx.send(nct).unwrap();
    });
    let mut nct = rx.iter().next().unwrap();
    let tex = nct.render(tex_creator.clone()).map_err(|e| e.to_string())?;
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
        canvas.copy(tex, None, None);
        canvas.present();
        sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
    return Ok(());
}