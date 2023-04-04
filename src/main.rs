use env_logger::Target;
use std::io::Write;

use crate::sdl_engine::SDLEngine;
use crate::utils::SDLLoggerPipe;

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
    std::env::set_var("RUST_LOG", "info");
    env_logger::builder()
        .format_timestamp(None)
        .format(|buf, record| {
            writeln!(buf, "{}: {}", record.target(), record.args())
        })
        .target(Target::Pipe(Box::new(SDLLoggerPipe)))
        .init();

    let rui_dsl = String::from("");
    SDLEngine::new_main_loop(rui_dsl)?;
    Ok(())
}
