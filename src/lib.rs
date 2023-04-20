// Needed for mod main;
#![allow(special_module_name)]


use log::info;
use sdl2::libc;

mod texture;
mod widgets;
mod window;
mod general;
mod main;
mod utils;
mod engines;

#[no_mangle]
pub extern fn SDL_main(_argc: libc::c_int, _argv: *const *const libc::c_char) -> libc::c_int {
    match main::main() {
        Ok(_) => { 0 }
        Err(e) => {
            info!("Error");
            info!("{}", e.to_string());
            -1
        }
    }
}