use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CrateTarget {
    pub package_name: String,
    pub target_description: String,
}

impl Display for CrateTarget {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let Self {
            package_name,
            target_description,
        } = self;

        write!(formatter, "{package_name} {target_description}")
    }
}
