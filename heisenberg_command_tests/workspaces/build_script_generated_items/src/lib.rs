#[cfg_attr(generated, path = "generated_module.rs")]
pub mod generated;

use crate::configured::Item;

#[cfg(generated)]
macro_rules! configured_macro {
    () => {};
}

configured_macro!();

include!(concat!(env!("OUT_DIR"), "/included.rs"));

include!(concat!(env!("OUT_DIR"), "/nested/deeper.rs"));

pub fn item() -> Item {
    crate::configured::Item
}

#[cfg(generated)]
pub mod configured {
    pub struct Item;
}
