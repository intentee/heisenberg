use crate::greeting::Greeting;

pub struct Local;

impl Greeting for &Local {
    fn greet(&self) -> u8 {
        2
    }
}

impl Greeting for Vec<Local> {
    fn greet(&self) -> u8 {
        3
    }
}
