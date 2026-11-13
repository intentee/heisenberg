use std::fmt;

const CHECKED: Result<u8, fmt::Error> = Err(fmt::Error);

#[must_use]
pub fn constant_check() -> bool {
    CHECKED.is_ok()
}
