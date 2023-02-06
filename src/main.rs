use crate::sdl_engine::SDLEngine;

mod texture;
mod components;
mod window;
mod sdl_engine;

/*
Start with one DSL, it could be empty, declare it old_dsl
Loop {
    Take the input
    Clone old_dsl to DSL and update new one according to the input (could this be multithreaded too?) including lib-user's actions
    build() (multithreaded)
        Compare DSL with old_dsl from previous frame
        then based on the deltas it will take the original resources and change them accordingly
        Process everything except for rendering and GPU texture manipulation
    render()
        render all resources and manipulate GPU textures
}
 */


fn main() -> Result<(), String> {
    println!("Hello, world!");
    let rui_dsl = String::from("");

    SDLEngine::main_loop(rui_dsl)?;
    println!("Bye, world!");
    return Ok(());
}
