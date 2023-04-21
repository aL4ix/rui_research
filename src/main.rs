use std::io::Write;
use std::path::Path;

use env_logger::Target;
use glyph_brush::ab_glyph::FontArc;

use crate::engines::sdl::SDLEngine;
use crate::general::{Color, Vector2D};
use crate::utils::{Assets, SDLLoggerPipe};
use crate::widgets::{Image, Shape, Text, Widget};
use crate::window::WindowBuilder;

/*
Start with one DSL, it could be empty, declare it old_dsl
Loop {
    Take the input, widgets should exist and match DSL, so input is just changing values in widgets,
    in a lazy way, later at build() changes will be applied.
    Clone old_dsl to DSL and update new one according to the input (could this be multithreaded too?)
        including lib-user's actions, i mean the events' programming, take the mouse and send it to
        the parent, then the parent check if the mouse is within its boundaries, if so send it to
        the children. and repeat. For the keyboard, check first globals like CTRL or ALT combinations
        then if not send it to the focused component and let that propagate it to its children.
        The idea is defaults will change DSL, and events' programming will too. No access to the
        built components. We need a different name for DSL compo and built compo. Maybe element,
        widget and geometry.

    build() (multithreaded)
        Compare DSL with old_dsl from previous frame
        then based on the deltas it will take the original resources and change them accordingly
        Process everything except for rendering and GPU texture manipulation
    render()
        render all resources and manipulate GPU textures
}
 */

/*
In reality the main() for the binary is inside bin.rs and for the library is in lib.rs, but both
end up calling this main()
 */
pub fn main() -> Result<(), Box<(dyn std::error::Error)>> {
    std::env::set_var("RUST_BACKTRACE", "full");
    // std::env::set_var("RUST_LOG", "info");
    env_logger::builder()
        .format_timestamp(None)
        .format(|buf, record| {
            writeln!(buf, "{}: {}", record.target(), record.args())
        })
        .target(Target::Pipe(Box::new(SDLLoggerPipe)))
        .init();

    let mut sdl_engine = SDLEngine::init()?;

    // Multi-threaded
    // let (tx, rx) = mpsc::channel();
    // thread::spawn(move || { // Lets test if it's possible to create widgets from other threads
    //     let image = Image::from_bmp(1, Box::from(Path::new("assets/image.bmp")));
    //
    //     tx.send(image).unwrap();
    // });
    // let image = rx.iter().next().unwrap();

    // Single-threaded
    let mut window_builder = WindowBuilder::new()?;
    let mut image = Image::from_bmp(1, Box::from(Path::new("assets/image.bmp")))?;
    image.set_position(Vector2D::new(0.0, 100.0));

    // TODO what to do with errors in widget constructors
    window_builder.add_widget(0, Box::new(image));

    let font_path = "assets/Nouveau_IBM.ttf";
    let font_vec = Assets::read(font_path)?;
    let font = FontArc::try_from_vec(font_vec)?;
    let text = Text::new(2, "RUI", 300.0, font.clone(),
                         Color::new(50, 50, 255, 200));
    window_builder.add_widget(2, Box::new(text));

    let mut shape = Shape::new_square(Vector2D::new(100.0, 50.0), 0,
                                      Color::new(255, 255, 255, 255));
    shape.set_position(Vector2D::new(100.0, 100.0));
    window_builder.add_widget(1, Box::from(shape));
    sdl_engine.add_window_builder(window_builder)?;
    // let mut w2 = WindowSpecs::new()?;
    // let t2 = Text::new(1, "w2", 30.0, font, Color::new(255, 255, 255, 128));
    // w2.add_widget(1, Box::new(t2));
    // sdl_engine.add_window(w2)?;

    // sdl_engine.set_user_event_handler(Some(|event| {
    //     log::info!("{:?}", event);
    //     crate::engines::sdl::MainLoopStatus::Continue
    // }));

    sdl_engine.main_loop();
    Ok(())
}
