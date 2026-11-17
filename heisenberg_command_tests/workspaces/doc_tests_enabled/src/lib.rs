/// Doubles a number.
///
/// ```
/// assert_eq!(doc_tests_enabled::doubled(2), 4);
/// ```
#[must_use]
pub const fn doubled(number: u8) -> u8 {
    number.saturating_mul(2)
}
