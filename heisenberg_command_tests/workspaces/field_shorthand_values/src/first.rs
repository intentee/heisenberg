use crate::declaration::Declaration;

const fn command() -> u8 {
    1
}

pub const fn first() -> Declaration {
    Declaration { command }
}
