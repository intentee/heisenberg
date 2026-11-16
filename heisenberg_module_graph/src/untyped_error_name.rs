use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UntypedErrorName {
    Named { error_type: String },
    NamedByAlias { alias: String, error_type: String },
}

impl Display for UntypedErrorName {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Named { error_type } => formatter.write_str(error_type),
            Self::NamedByAlias { alias, error_type } => {
                write!(formatter, "{error_type}, which the alias {alias} names,")
            }
        }
    }
}
