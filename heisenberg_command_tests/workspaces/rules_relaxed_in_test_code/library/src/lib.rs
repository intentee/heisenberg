pub mod pair;
pub mod report;

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fmt;
    use std::fmt::Display;
    use std::fmt::Formatter;

    use crate::pair::Pair;
    use crate::report::Report;

    macro_rules! unit_failing {
        ($failure:ident) => {
            impl Error for $failure {}
        };
    }

    unit_failing!(UnitGenerated);

    fn unit_unreadable(formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str("the unit key cannot be read")
    }

    fn unit_erased() -> Result<(), Box<dyn Error>> {
        Err(Box::new(UnitStub))
    }

    #[derive(Debug)]
    struct UnitStub;

    impl Display for UnitStub {
        fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
            formatter.write_str("the unit stub failed")
        }
    }

    impl Error for UnitStub {}

    #[derive(Debug)]
    struct UnitGenerated;

    impl Display for UnitGenerated {
        fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
            formatter.write_str("the unit generated failure")
        }
    }

    #[derive(Debug, thiserror::Error)]
    #[error("the unit check failed")]
    enum UnitFailure {
        Missing,
        #[error(fmt = unit_unreadable)]
        Unreadable,
        #[error("the unit lookup failed: {source}")]
        Lookup {
            #[source]
            source: UnitStub,
        },
        #[error("the unit fallback failed")]
        Fallback { cause: UnitStub },
    }

    #[derive(Debug, thiserror::Error)]
    #[error("the unit marker failed")]
    struct UnitMarker;

    impl Pair<UnitStub, UnitGenerated> for u32 {}

    impl Report {
        const fn unit_doubled(&self) -> u8 {
            self.lines.saturating_mul(2)
        }
    }

    #[test]
    fn unit_failures_are_rendered() {
        assert_eq!(UnitFailure::Missing.to_string(), "the unit check failed");
        assert_eq!(UnitFailure::Unreadable.to_string(), "the unit key cannot be read");
        assert_eq!(
            UnitFailure::Lookup { source: UnitStub }.to_string(),
            "the unit lookup failed: the unit stub failed"
        );
        assert_eq!(
            UnitFailure::Fallback { cause: UnitStub }.to_string(),
            "the unit fallback failed"
        );
        assert_eq!(UnitMarker.to_string(), "the unit marker failed");
        assert_eq!(UnitGenerated.to_string(), "the unit generated failure");
        assert!(unit_erased().is_err());
        assert_eq!(Report { lines: 2 }.unit_doubled(), 4);
    }
}
