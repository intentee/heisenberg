use std::fmt::Display;

pub fn versioned_name(name: &str, version: &impl Display) -> String {
    format!("{name} v{version}")
}
