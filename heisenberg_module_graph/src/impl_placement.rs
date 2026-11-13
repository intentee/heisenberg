use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ImplPlacement {
    Blanket {
        trait_file: String,
    },
    ForeignType {
        trait_file: String,
        trait_name: String,
    },
    TraitObject {
        trait_file: String,
    },
    Type {
        type_file: String,
    },
}

impl Display for ImplPlacement {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Blanket { trait_file } => {
                write!(formatter, "Blanket impl belongs in {trait_file}")
            }
            Self::ForeignType {
                trait_file,
                trait_name,
            } => write!(
                formatter,
                "Impl block of {trait_name} for a foreign type belongs in {trait_file}"
            ),
            Self::TraitObject { trait_file } => {
                write!(
                    formatter,
                    "Impl block on a trait object belongs in {trait_file}"
                )
            }
            Self::Type { type_file } => write!(formatter, "Impl block belongs in {type_file}"),
        }
    }
}
