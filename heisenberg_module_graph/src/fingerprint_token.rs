use crate::definition_identity::DefinitionIdentity;
use crate::token_spelling::TokenSpelling;
use crate::written_offset::WrittenOffset;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum FingerprintToken {
    Definitions(Vec<DefinitionIdentity>),
    FormatTemplate {
        captures: Vec<DefinitionIdentity>,
        spelling: TokenSpelling,
    },
    Spelling(TokenSpelling),
    Unresolved(WrittenOffset),
}
