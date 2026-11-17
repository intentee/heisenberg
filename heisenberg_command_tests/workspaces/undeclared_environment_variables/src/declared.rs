#[must_use]
pub fn declared() -> [Option<&'static str>; 4] {
    [
        Some(env!("CARGO_PKG_NAME")),
        Some(env!("CONFIGURED_FOR_THE_FIXTURE")),
        Some(env!("EMITTED_FOR_THE_FIXTURE")),
        option_env!("CONFIGURED_FOR_THE_FIXTURE"),
    ]
}
