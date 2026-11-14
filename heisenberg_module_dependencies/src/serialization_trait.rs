use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::constants::DESERIALIZE_OWNED_TRAIT_PATH;
use crate::constants::DESERIALIZE_TRAIT_PATH;
use crate::constants::SERIALIZE_TRAIT_PATH;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SerializationTrait {
    Deserialize,
    DeserializeOwned,
    Serialize,
}

impl SerializationTrait {
    #[must_use]
    pub const fn path(self) -> [&'static str; 2] {
        match self {
            Self::Deserialize => DESERIALIZE_TRAIT_PATH,
            Self::DeserializeOwned => DESERIALIZE_OWNED_TRAIT_PATH,
            Self::Serialize => SERIALIZE_TRAIT_PATH,
        }
    }
}

impl Display for SerializationTrait {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.path().join("::"))
    }
}
