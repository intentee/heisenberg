use crate::constants::LIMIT;
use crate::detail::Detail;

#[must_use]
pub const fn detailed() -> Detail {
    Detail { depth: LIMIT }
}
