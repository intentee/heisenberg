use crate::constants::UNIT;
use shapes::square::Square;
use std::fmt::Display;

pub fn area(square: &Square) -> impl Display {
    format!("{}{UNIT}", square.area())
}
