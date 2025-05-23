use crate::general::Vector2D;

#[derive(Clone, Debug, Copy)]
pub struct Rect<T> {
    x: T,
    y: T,
    width: T,
    height: T,
}

impl<T: std::ops::Add<Output = T> + PartialOrd + Copy + Default> Rect<T> {
    pub fn new(x: T, y: T, width: T, height: T) -> Rect<T> {
        Rect {
            x,
            y,
            width,
            height,
        }
    }
    pub fn new_zero() -> Rect<T> {
        Self::new(
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        )
    }
    pub fn bottom(&self) -> T {
        self.y + self.height
    }
    pub fn right(&self) -> T {
        self.x + self.width
    }
    pub fn x(&self) -> T {
        self.x
    }
    pub fn y(&self) -> T {
        self.y
    }
    pub fn width(&self) -> T {
        self.width
    }
    pub fn height(&self) -> T {
        self.height
    }
    pub fn unpack(&self) -> (T, T, T, T) {
        (self.x, self.y, self.width, self.height)
    }
    pub fn contains_point(&self, point: Vector2D<T>) -> bool {
        if self.x() < point.x()
            && point.x() < self.right()
            && self.y() < point.y()
            && point.y() < self.bottom()
        {
            return true;
        }
        false
    }
}

impl From<Rect<u32>> for Option<sdl2::rect::Rect> {
    fn from(val: Rect<u32>) -> Self {
        Some(sdl2::rect::Rect::new(
            val.x().try_into().expect("rect:From:from"),
            val.y().try_into().expect("rect:From:from"),
            val.width(),
            val.height(),
        ))
    }
}
