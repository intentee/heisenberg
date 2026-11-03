pub trait Greeting {
    fn greet(&self) -> u8;
}

impl Greeting for String {
    fn greet(&self) -> u8 {
        1
    }
}
