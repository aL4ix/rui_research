// Needed for mod main;
#![allow(special_module_name)]


mod texture;
mod widgets;
mod window;
mod general;
mod main;
mod utils;
mod engines;

fn main() -> Result<(), Box<(dyn std::error::Error)>> {
    main::main()
}
