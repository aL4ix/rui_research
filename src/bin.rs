// Needed for mod main;
#![allow(special_module_name)]


mod texture;
mod widgets;
mod window;
mod sdl_engine;
mod general;
mod main;
mod utils;

fn main() -> Result<(), Box<(dyn std::error::Error)>> {
    main::main()
}
