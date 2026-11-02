use std::error::Error;
use std::iter;

pub fn error_chain<TError: Error + 'static>(error: &TError) -> Vec<String> {
    iter::successors(Some(error as &dyn Error), |cause: &&dyn Error| {
        (*cause).source()
    })
    .map(ToString::to_string)
    .collect()
}
