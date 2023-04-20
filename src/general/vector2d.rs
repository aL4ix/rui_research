#[derive(Clone, Debug)]
pub struct Vector2D<T> {
    x: T,
    y: T,
}

impl<T: Copy + Default> Vector2D<T> {
    pub fn new(x: T, y: T) -> Vector2D<T> {
        Vector2D {
            x,
            y,
        }
    }
    pub fn x(&self) -> T {
        self.x
    }
    pub fn y(&self) -> T {
        self.y
    }
    pub fn unpack(&self) -> (T, T) {
        (self.x, self.y)
    }
}

impl<T: Copy + Default> Default for Vector2D<T> {
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}