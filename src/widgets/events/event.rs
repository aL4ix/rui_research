use std::fmt::{Debug, Formatter};

pub trait Event: Debug {
    fn dyn_fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {{}}", self.class())
    }
    fn class(&self) -> &str;
}
