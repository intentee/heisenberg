use crate::source_position::SourcePosition;
use crate::token_spelling::TokenSpelling;
use crate::written_offset::WrittenOffset;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DefinitionIdentity {
    Builtin {
        name: Option<String>,
        spelling: TokenSpelling,
    },
    CrateLevel {
        crate_root: String,
        name: Option<String>,
        spelling: TokenSpelling,
    },
    Declared {
        index: usize,
    },
    Impl(SourcePosition),
    Member {
        container: Box<Self>,
        name: Option<String>,
    },
    Written(WrittenOffset),
}
