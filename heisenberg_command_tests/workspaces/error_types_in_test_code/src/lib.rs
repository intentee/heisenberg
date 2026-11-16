pub mod lookup_failure;

#[cfg(test)]
mod tests {
    use crate::lookup_failure::LookupFailure;

    #[derive(Debug, thiserror::Error)]
    enum ExpectationFailure {
        #[error("the lookup failed")]
        Lookup(#[from] LookupFailure),
    }

    #[test]
    fn expectation_failure_wraps_a_lookup_failure() {
        assert_eq!(
            ExpectationFailure::from(LookupFailure::Missing).to_string(),
            "the lookup failed"
        );
    }
}
