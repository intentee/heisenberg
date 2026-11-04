macro_rules! held_name {
    () => {
        "HELD_UNDECLARED"
    };
}

macro_rules! read_variable {
    ($name:expr) => {
        env!($name)
    };
}

pub mod constants;
