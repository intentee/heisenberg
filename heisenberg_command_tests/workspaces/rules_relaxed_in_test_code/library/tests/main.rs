use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

macro_rules! target_failing {
    ($failure:ident) => {
        impl Error for $failure {}
    };
}

target_failing!(TargetGenerated);

fn target_unreadable(formatter: &mut Formatter<'_>) -> fmt::Result {
    formatter.write_str("the target key cannot be read")
}

fn target_erased() -> Result<(), Box<dyn Error>> {
    Err(Box::new(TargetStub))
}

#[derive(Debug)]
struct TargetStub;

impl Display for TargetStub {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str("the target stub failed")
    }
}

impl Error for TargetStub {}

#[derive(Debug)]
struct TargetGenerated;

impl Display for TargetGenerated {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str("the target generated failure")
    }
}

#[derive(Debug, thiserror::Error)]
#[error("the target check failed")]
enum TargetFailure {
    Missing,
    #[error(fmt = target_unreadable)]
    Unreadable,
    #[error("the target lookup failed: {source}")]
    Lookup {
        #[source]
        source: TargetStub,
    },
    #[error("the target fallback failed")]
    Fallback { cause: TargetStub },
}

#[derive(Debug, thiserror::Error)]
#[error("the target marker failed")]
struct TargetMarker;

#[test]
fn target_failures_are_rendered() {
    assert_eq!(TargetFailure::Missing.to_string(), "the target check failed");
    assert_eq!(TargetFailure::Unreadable.to_string(), "the target key cannot be read");
    assert_eq!(
        TargetFailure::Lookup { source: TargetStub }.to_string(),
        "the target lookup failed: the target stub failed"
    );
    assert_eq!(
        TargetFailure::Fallback { cause: TargetStub }.to_string(),
        "the target fallback failed"
    );
    assert_eq!(TargetMarker.to_string(), "the target marker failed");
    assert_eq!(TargetGenerated.to_string(), "the target generated failure");
    assert!(target_erased().is_err());
}
