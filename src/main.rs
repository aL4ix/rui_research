use crate::sdl_engine::SDLEngine;

mod texture;
mod widgets;
mod window;
mod sdl_engine;
mod tex_man;
mod general;

/*
Start with one DSL, it could be empty, declare it old_dsl
Loop {
    Take the input
    Clone old_dsl to DSL and update new one according to the input (could this be multithreaded too?)
        including lib-user's actions, i mean the events' programming, take the mouse and send it to
        the parent, then the parent check if the mouse is within its boundaries, if so send it to
        the children. and repeat. For the keyboard, check first globals like CTRL or ALT combinations
        then if not send it to the focused component and let that propagate it to its children.
        The idea is defaults will change DSL, and events' programming will too. No access to the
        built components. We need a different name for DSL compo and built compo. Maybe element,
        widget and body.

    build() (multithreaded)
        Compare DSL with old_dsl from previous frame
        then based on the deltas it will take the original resources and change them accordingly
        Process everything except for rendering and GPU texture manipulation
    render()
        render all resources and manipulate GPU textures
}
 */

fn main() -> Result<(), Box<(dyn std::error::Error)>> {
    let rui_dsl = String::from("");
    SDLEngine::main_loop(rui_dsl)?;
    return Ok(());
}
