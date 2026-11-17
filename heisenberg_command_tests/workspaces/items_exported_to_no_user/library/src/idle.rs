use crate::report::Report;

#[must_use]
pub const fn idle() -> u8 {
    Report { lines: 1, pages: 2 }.doubled()
}
