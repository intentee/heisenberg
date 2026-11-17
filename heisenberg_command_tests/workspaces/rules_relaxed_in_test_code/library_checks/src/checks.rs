use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::shape::Shape;

macro_rules! crate_failing {
    ($failure:ident) => {
        impl Error for $failure {}
    };
}

crate_failing!(CrateGenerated);

fn crate_unreadable(formatter: &mut Formatter<'_>) -> fmt::Result {
    formatter.write_str("the crate key cannot be read")
}

pub fn crate_erased() -> Result<(), Box<dyn Error>> {
    Err(Box::new(CrateStub))
}

#[derive(Debug)]
pub struct CrateStub;

impl Display for CrateStub {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str("the crate stub failed")
    }
}

impl Error for CrateStub {}

#[derive(Debug)]
pub struct CrateGenerated;

impl Display for CrateGenerated {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str("the crate generated failure")
    }
}

#[derive(Debug, thiserror::Error)]
#[error("the crate check failed")]
pub enum CrateFailure {
    Missing,
    #[error(fmt = crate_unreadable)]
    Unreadable,
    #[error("the crate lookup failed: {source}")]
    Lookup {
        #[source]
        source: CrateStub,
    },
    #[error("the crate fallback failed")]
    Fallback { cause: CrateStub },
}

#[derive(Debug, thiserror::Error)]
#[error("the crate marker failed")]
pub struct CrateMarker;

pub const LIMIT: u8 = 3;

impl Shape {
    pub const CORNERS: u8 = 4;

    #[must_use]
    pub const fn doubled(&self) -> u8 {
        self.sides.saturating_mul(2)
    }
}
