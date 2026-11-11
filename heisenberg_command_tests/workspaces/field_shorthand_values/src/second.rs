use crate::declaration::Declaration;

const fn command() -> u8 {
    2
}

pub const fn second() -> Declaration {
    Declaration { command }
}
