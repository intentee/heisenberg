#![feature(macro_derive)]

macro_rules! described {
    derive() ($visibility:vis struct $name:ident;) => {
        impl crate::second::Second for $name {}
    };
}

pub mod first;
pub mod second;
