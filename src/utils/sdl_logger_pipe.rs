use std::io::{ErrorKind, Write};

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
