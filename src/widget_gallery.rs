use std::io::Write;
use std::path::Path;

use env_logger::Target;
use log::info;

use crate::engines::sdl::SDLEngine;
use crate::themes::{DarkSimpleTheme, StyleMaster};
use crate::utils::SDLLoggerPipe;
use crate::widgets::events::HasEvents;
use crate::widgets::{Button, Compound, Image, TextBox, Widget, WidgetEnum, WidgetId};
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

#[repr(usize)]
#[derive(Clone, Copy, Debug)]
pub enum WidgetGalleryEnum {
    IMAGE,
    TEXTBOX,
    BUTTON,
    TEXTBOX2,
    CUSTOM,
}

impl WidgetEnum for WidgetGalleryEnum {
    fn to_wid(self) -> WidgetId {
        self as WidgetId
    }
}

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
    let style_master = StyleMaster::new(Box::new(DarkSimpleTheme))?;
    // Can we have a global theme instead of sending it to each widget?
    let mut window_builder = WindowBuilder::new()?;
    let mut image = Image::from_bmp(
        WidgetGalleryEnum::IMAGE,
        Box::from(Path::new("assets/image.bmp")),
        style_master.clone(),
    )?;
    image.set_event_key_down(|root, keycode| {
        TextBox::get_by_id(root, WidgetGalleryEnum::TEXTBOX)
            .expect("widget_gallery:main:image.set_event_key_down")
            .lock()
            .expect("set_event_key_down")
            .set_text(&keycode.to_string());
    });
    // TODO what to do with errors in widget constructors, first organize all errors in all the traits
    window_builder.add_widget(0, image);

    let text_box = TextBox::new(WidgetGalleryEnum::TEXTBOX, "RUI", style_master.clone())?;

    let mut button = Button::new(WidgetGalleryEnum::BUTTON, "button", style_master.clone())?;
    button.set_event_mouse_button_down(|root, x, y| {
        info!("Button.set_event Clicked! {} {}", x, y);

        let btn = Button::get_by_id(root, WidgetGalleryEnum::BUTTON)
            .expect("widget_gallery:main:button.set_event_key_down");
        btn.lock()
            .expect("set_event_mouse_button_down")
            .set_text(&format!("Clicked {} {}", x, y));

        let tx = TextBox::get_by_id(root, WidgetGalleryEnum::TEXTBOX)
            .expect("widget_gallery:main:button.set_event_key_down");
        tx.lock()
            .expect("set_event_mouse_button_down")
            .set_text("Mickey es gason");
    });

    let mut compound = Compound::new(WidgetGalleryEnum::CUSTOM, style_master)?;
    compound.add_widget(button);
    compound.add_widget(text_box);
    window_builder.add_widget(5, compound);

    sdl_engine.add_window_builder(window_builder)?;

    // let mut w2 = WindowBuilder::new()?;
    // let t2 = TextBox::new(0, "w2", style_master)?;
    // w2.add_widget(1, t2, WidgetGalleryEnum::TEXTBOX2);
    // sdl_engine.add_window_builder(w2)?;
    // sdl_engine.set_user_event_handler(Some(|event| {
    //     log::info!("{:?}", event);
    //     crate::engines::sdl::MainLoopStatus::Continue
    // }));

    sdl_engine.main_loop();
    Ok(())
}
