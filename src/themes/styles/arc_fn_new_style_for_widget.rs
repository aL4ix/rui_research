use std::{any::type_name, error::Error, fmt, sync::Arc};

use crate::themes::{PropertiesMap, StyleForWidget};

pub type ArcFnNewStyleForWidget =
    Arc<dyn Fn(PropertiesMap) -> Result<Box<dyn StyleForWidget>, Box<dyn Error>> + Send + Sync>;

// Why do we need a wrapper? (Wrap because it's too long already, that's what she said)
// That `dyn Fn()` does not implement Debug automatically, so we need a wrapper to implement it so
// it can play nicely with the rest of the structs.
pub struct ArcFnNewStyleForWidgetWrap(
    pub Arc<dyn Fn(PropertiesMap) -> Result<Box<dyn StyleForWidget>, Box<dyn Error>> + Send + Sync>,
);

impl fmt::Debug for ArcFnNewStyleForWidgetWrap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<Fn: {}>",
            type_name::<
                dyn Fn(PropertiesMap) -> Result<Box<dyn StyleForWidget>, Box<dyn Error>>
                    + Send
                    + Sync,
            >()
        )
    }
}
