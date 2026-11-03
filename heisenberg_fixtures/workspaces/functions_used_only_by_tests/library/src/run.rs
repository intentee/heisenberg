use crate::counter::Counter;

pub const fn run() -> u8 {
    let mut counter = Counter::new();

    counter.increment();

    counter.value
}
