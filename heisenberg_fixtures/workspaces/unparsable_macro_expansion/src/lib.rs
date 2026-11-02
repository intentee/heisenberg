#![feature(macro_attr, macro_derive)]

macro_rules! unparsable_expansion {
    () => {
        struct
    };
}

unparsable_expansion!();

macro_rules! unparsable_impl {
    () => {
        impl
    };
}

unparsable_impl!();

macro_rules! unparsable_attribute {
    attr() ($item:item) => {
        fn
    };
}

#[unparsable_attribute]
pub struct Rewritten;

macro_rules! unparsable_derive {
    derive() ($item:item) => {
        fn
    };
}

#[derive(unparsable_derive)]
struct Derived;
