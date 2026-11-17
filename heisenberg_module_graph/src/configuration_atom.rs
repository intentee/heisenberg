use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConfigurationAtom {
    Flag { name: String },
    KeyValue { key: String, value: String },
}

impl Display for ConfigurationAtom {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Flag { name } => formatter.write_str(name),
            Self::KeyValue { key, value } => write!(formatter, "{key} = \"{value}\""),
        }
    }
}
