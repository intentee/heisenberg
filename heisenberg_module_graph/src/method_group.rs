use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MethodGroup {
    Constructor,
    Public,
    Crate,
    Restricted,
    Private,
    Trait,
}

impl Display for MethodGroup {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Constructor => "constructors",
            Self::Public => "public methods",
            Self::Crate => "crate methods",
            Self::Restricted => "restricted methods",
            Self::Private => "private methods",
            Self::Trait => "trait methods",
        })
    }
}
