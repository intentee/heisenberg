rust_analyzer_divergent_macros::hidden_from_rust_analyzer! {
    use identity_marker_macros::service as renamed;
}

#[renamed]
#[identity_marker_macros::singleton]
pub fn one() -> u32 {
    1
}
