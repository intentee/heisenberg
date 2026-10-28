#[cfg(generated)]
pub mod configured {
    pub struct Item;
}

macro_rules! configured_item {
    () => {
        crate::configured::Item
    };
}

pub fn item() {
    let _item = configured_item!();
}
