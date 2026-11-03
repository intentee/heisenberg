pub struct Settings;

pub mod constants {
    pub const DEFAULT: u8 = 6;
}

macro_rules! generated_constant {
    () => {
        pub const GENERATED: u8 = 7;
    };
}

generated_constant!();
