use std::error::Error;
use std::io::{ErrorKind, Read, Write};
use std::path::Path;

use log::{debug};

pub struct SDLLoggerPipe;

impl Write for SDLLoggerPipe {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let utf8 = std::str::from_utf8(buf);
        match utf8 {
            Ok(s) => {
                // SDL_log will truncate anything after 4096, Android's logcat won't display it
                // after a little longer, how long, IDK.
                sdl2::log::log(s);
                Ok(buf.len())
            }
            Err(_) => Err(std::io::Error::new(ErrorKind::InvalidInput, "Not UTF8"))
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

unsafe impl Send for SDLLoggerPipe {}

///////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Assets {}

impl Assets {
    pub fn root_directory() -> &'static str {
        "\"assets"
        // TODO why \"
    }

    pub fn read<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Box<dyn Error>> {
        let root_directory = Self::root_directory();
        if path.as_ref().starts_with(root_directory) {
            return Err(Box::from(format!("Path should start with {}: {:?}", root_directory, path.as_ref())));
        }
        debug!("Attempting to load asset {:?}", path.as_ref());
        let mut file = sdl2::rwops::RWops::from_file(path, "rb")?;
        let l = file.len().unwrap_or(0);
        let mut v = Vec::with_capacity(l);
        file.read_to_end(&mut v)?;
        debug!("Asset loaded successfully");
        Ok(v)
    }
}