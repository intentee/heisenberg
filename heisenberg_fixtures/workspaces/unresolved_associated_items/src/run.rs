use crate::item::Item;

pub fn run() {
    Item::missing();
    Item.absent();
    missing_function().unknown();
    Item.absent().unknown();
}
