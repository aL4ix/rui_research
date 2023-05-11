// Needed for mod main;
#![allow(special_module_name)]
// https://github.com/chris-morgan/mopa/issues/11
#![allow(clippy::transmute_ptr_to_ref)]
extern crate mopa;

mod engines;
mod general;
mod main;
mod texture;
mod utils;
mod widgets;
mod window;

fn main() -> Result<(), Box<(dyn std::error::Error)>> {
    main::main()
}
