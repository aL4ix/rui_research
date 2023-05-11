// Needed for mod main;
#![allow(special_module_name)]
// https://github.com/chris-morgan/mopa/issues/11
#![allow(clippy::transmute_ptr_to_ref)]
extern crate mopa;

use log::info;
use sdl2::libc;

mod engines;
mod general;
mod main;
mod texture;
mod utils;
mod widgets;
mod window;

#[no_mangle]
pub extern "C" fn SDL_main(_argc: libc::c_int, _argv: *const *const libc::c_char) -> libc::c_int {
    match main::main() {
        Ok(_) => 0,
        Err(e) => {
            info!("Error");
            info!("{}", e.to_string());
            -1
        }
    }
}
