// https://github.com/chris-morgan/mopa/issues/11
#![allow(clippy::transmute_ptr_to_ref)]

use log::info;
use sdl2::libc;

pub mod engines;
pub mod general;
pub mod texture;
pub mod themes;
pub mod utils;
pub mod widget_gallery;
pub mod widgets;
pub mod window;

#[no_mangle]
pub extern "C" fn SDL_main(_argc: libc::c_int, _argv: *const *const libc::c_char) -> libc::c_int {
    match widget_gallery::main() {
        Ok(_) => 0,
        Err(e) => {
            info!("Error");
            info!("{}", e.to_string());
            -1
        }
    }
}
