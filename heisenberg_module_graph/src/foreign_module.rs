use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ForeignModule {
    pub file: String,
    pub name: String,
}

impl Display for ForeignModule {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.name)
    }
}
