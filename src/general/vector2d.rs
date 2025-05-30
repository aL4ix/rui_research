use super::Size2D;

#[derive(Clone, Debug)]
pub struct Vector2D<T> {
    x: T,
    y: T,
}

impl<T: Copy + Default> Vector2D<T> {
    pub fn new(x: T, y: T) -> Vector2D<T> {
        Vector2D { x, y }
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

impl<T: Copy + Default> From<&(T, T)> for Vector2D<T> {
    fn from(v: &(T, T)) -> Self {
        Vector2D::new(v.0, v.1)
    }
}

impl From<Size2D> for Vector2D<f32> {
    fn from(v: Size2D) -> Self {
        Vector2D::new(v.width as f32, v.height as f32)
    }
}
