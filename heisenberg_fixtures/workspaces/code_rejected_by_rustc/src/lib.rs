extern crate missing_dependency;

macro_rules! invalid_definition {
    ($value) => {};
}

macro_rules! unparsable_expansion {
    () => {
        struct
    };
}

unparsable_expansion!();

#[rustc_builtin_macro]
macro_rules! unknown_builtin {
    () => {};
}

#[cfg(not(first, second))]
pub fn invalid_predicate() {}
