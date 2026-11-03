use crate::counter::Counter;

pub const fn run() -> u8 {
    Counter { value: 1 }.incremented()
}
