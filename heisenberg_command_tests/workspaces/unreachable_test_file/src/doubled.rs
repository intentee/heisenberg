#[must_use]
pub const fn doubled(value: u8) -> u8 {
    value.saturating_mul(2)
}
