rust_analyzer_divergent_macros::hidden_from_rust_analyzer! {
    macro_rules! one {
        () => {
            pub fn one() -> u32 {
                1
            }
        };
    }
}

one!();
