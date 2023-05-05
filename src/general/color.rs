#[derive(Clone, Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    /// Alpha of 255 is opaque, 0 is transparent
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    pub fn new_opaque(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
    pub fn r(&self) -> u8 {
        self.r
    }
    pub fn g(&self) -> u8 {
        self.g
    }
    pub fn b(&self) -> u8 {
        self.b
    }
    pub fn a(&self) -> u8 {
        self.a
    }
}

impl From<&Vec<u8>> for Color {
    fn from(v: &Vec<u8>) -> Self {
        let l = v.len();
        if l == 3 {
            Color::new_opaque(v[0], v[1], v[2])
        }
        else if l == 4 {
            Color::new(v[0], v[1], v[2], v[3])
        }
        else {
            panic!("Cannot convert Vec of length {} to Color", l)
            // TODO move to compile_error!
        }
    }
}