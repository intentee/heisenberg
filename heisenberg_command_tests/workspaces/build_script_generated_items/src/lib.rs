#[cfg_attr(generated, path = "generated_module.rs")]
pub mod generated;

#[cfg(generated)]
pub mod configured {
    pub struct Item;
}

#[cfg(generated)]
macro_rules! configured_macro {
    () => {};
}

use crate::configured::Item;

configured_macro!();

include!(concat!(env!("OUT_DIR"), "/included.rs"));

include!(concat!(env!("OUT_DIR"), "/nested/deeper.rs"));

pub fn item() -> Item {
    crate::configured::Item
}
