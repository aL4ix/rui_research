use std::io::Write;
use std::path::Path;

use env_logger::Target;
use log::info;

use crate::engines::sdl::SDLEngine;
use crate::general::Vector2D;
use crate::utils::SDLLoggerPipe;
use crate::widgets::themes::{SimpleTheme, StyleMaster};
use crate::widgets::{Button, Image, Primitive, TextBox, Widget};
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

pub fn main() -> Result<(), Box<(dyn std::error::Error)>> {
    std::env::set_var("RUST_BACKTRACE", "full");
    // std::env::set_var("RUST_LOG", "info");
    env_logger::builder()
        .format_timestamp(None)
        .format(|buf, record| writeln!(buf, "{}: {}", record.target(), record.args()))
        .target(Target::Pipe(Box::new(SDLLoggerPipe))) // TODO Doesn't work with emscripten
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
    let style = StyleMaster::new(Box::new(SimpleTheme))?;
    // Can we have a global theme instead of sending it to each widget?
    let mut window_builder = WindowBuilder::new()?;
    let mut image = Image::from_bmp(0, Box::from(Path::new("assets/image.bmp")), &style)?;
    image.set_position(Vector2D::new(0.0, 100.0));
    // TODO what to do with errors in widget constructors, first organize all errors in all the traits
    window_builder.add_widget(0, image, 1);

    let text_box = TextBox::new(0, "RUI", &style)?;
    window_builder.add_widget(2, text_box, 2);

    let mut button = Button::new(0, "button", &style)?;
    button.set_event_mouse_button_down(|root, x, y| {
        // Image 1
        // Textbox 2
        // Button 3
        info!("Clicked! {} {}", x, y);

        let btn = Button::get_by_id(root, 3).expect("Nel");
        btn.lock()
            .expect("widget_gallery:main:set_event_mouse_button_down")
            .set_text(&format!("Clicked {} {}", x, y));

        let tx = TextBox::get_by_id(root, 2).expect("Nel");
        tx.lock().expect("widget_gallery:main:set_event_mouse_button_down").set_text("Mickey es gason");
    });
    window_builder.add_widget(5, button, 3);

    sdl_engine.add_window_builder(window_builder)?;
    // let mut w2 = WindowBuilder::new()?;
    // let t2 = Text::new(0, "w2", 30.0, font, Color::new(255, 255, 255, 128));
    // w2.add_widget(1, Box::new(t2));
    // sdl_engine.add_window_builder(w2)?;

    // sdl_engine.set_user_event_handler(Some(|event| {
    //     log::info!("{:?}", event);
    //     crate::engines::sdl::MainLoopStatus::Continue
    // }));

    sdl_engine.main_loop();
    Ok(())
}
