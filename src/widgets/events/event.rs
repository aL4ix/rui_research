use std::fmt::{Debug, Formatter};
use std::sync::Arc;

pub trait Event: Debug {
    fn dyn_fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {{}}", self.class())
    }
    fn class(&self) -> &str;
    type Callback;
    fn clone_callback(&self) -> Arc<Self::Callback>;
}
