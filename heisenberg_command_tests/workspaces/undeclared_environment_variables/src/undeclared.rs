#[must_use]
pub fn undeclared() -> [Option<&'static str>; 3] {
    [
        Some(env!("UNDECLARED_BY_THE_FIXTURE")),
        option_env!("OPTIONAL_AND_UNDECLARED_BY_THE_FIXTURE"),
        Some(env!("CARGO_BIN_NAME")),
    ]
}
