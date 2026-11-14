use std::error::Error;
use std::iter;

#[must_use]
pub fn error_chain(error: &(dyn Error + 'static)) -> Vec<String> {
    iter::successors(Some(error), |cause| (*cause).source())
        .map(ToString::to_string)
        .collect()
}
