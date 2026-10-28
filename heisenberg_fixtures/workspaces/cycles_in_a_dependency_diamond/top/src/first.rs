use crate::second::Second;

pub struct First {
    pub left: left::first::First,
    pub right: right::first::First,
    pub second: Option<Box<Second>>,
}
