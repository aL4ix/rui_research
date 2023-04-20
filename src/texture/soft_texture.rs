use std::cell::RefCell;
use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use crate::general::Polygon;
use crate::texture::TextureManager;

/// All Textures are lazy, because they can only be created or updated during render time.
/// https://documentation.help/SDL/thread.html
/// Note tex_creator and tex_man are references, which means it disallows multi-threaded usages.
/// All Textures are static, once one is created it won't change. Basically at the first time it
/// gets rendered it creates the actual texture and then after than it will never change.
pub trait SoftTexture: Send {
    fn id(&self) -> usize;
    fn render(&mut self, tex_creator: &TextureCreator<WindowContext>, tex_man: &mut TextureManager)
              -> Result<Rc<RefCell<Texture>>, Box<dyn Error>>;
    fn class(&self) -> &str;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn poly(&self) -> &Polygon;
    fn fmt_dyn(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let id = format!("{:?}", self.id());
        let class = format!("{:?}", self.class());
        let width = format!("{:?}", self.width());
        let height = format!("{:?}", self.height());
        let poly = format!("{:?}", self.poly());
        write!(f, "SoftTexture {{ id: {}, class: {}, width: {}, height: {}, poly: {} }}",
               id, class, width, height, poly)
    }
}

impl Debug for dyn SoftTexture {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_dyn(f)
    }
}
