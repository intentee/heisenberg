#![cfg_attr(not(feature = "optional"), allow(dead_code))]

pub struct Configured;

impl Configured {
    #[cfg(any(windows, unix))]
    pub fn any_platform() {}

    #[cfg_attr(not(test), must_use)]
    pub fn attributed() -> u8 {
        1
    }

    #[cfg(any(windows, all(unix, not(debug_assertions))))]
    pub fn debug_assertions_negated_off_windows() {}

    #[cfg(any(windows, not(feature = "optional")))]
    pub fn feature_negated_off_windows() {}

    #[cfg_attr(unix, cfg(not(feature = "optional")))]
    pub fn feature_negated_on_unix() {}

    #[cfg(not(first, second))]
    pub fn invalid_predicate() {}

    pub fn negated_statement() {
        #[cfg(not(feature = "optional"))]
        println!("negated");
    }

    #[cfg(not(windows))]
    pub fn not_windows() {}
}
