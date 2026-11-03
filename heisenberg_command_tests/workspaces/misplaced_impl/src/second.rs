use crate::first::First;
use crate::greeting::Greeting;

impl First {
    pub fn inherent(&self) -> u8 {
        1
    }
}

impl Greeting for First {
    fn greet(&self) -> u8 {
        2
    }
}
