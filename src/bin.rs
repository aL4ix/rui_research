// Needed for mod main;
#![allow(special_module_name)]
// https://github.com/chris-morgan/mopa/issues/11
#![allow(clippy::transmute_ptr_to_ref)]
extern crate mopa;


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
