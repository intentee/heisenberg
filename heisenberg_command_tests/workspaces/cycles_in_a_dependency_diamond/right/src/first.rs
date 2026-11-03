use crate::second::Second;

pub struct First {
    pub bottom: bottom::first::First,
    pub second: Option<Box<Second>>,
}
