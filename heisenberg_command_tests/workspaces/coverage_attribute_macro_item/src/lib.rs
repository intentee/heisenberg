use identity_marker_macros::service;

#[must_use]
#[service]
pub fn describe(value: u32) -> String {
    if value > 10 { format!("large {value}") } else { String::from("small") }
}

#[service]
pub trait Described {
    fn described(&self) -> String;
}
