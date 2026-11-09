use crate::definition_identity::DefinitionIdentity;
use crate::token_spelling::TokenSpelling;
use crate::written_offset::WrittenOffset;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum FingerprintToken {
    Definitions(Vec<DefinitionIdentity>),
    Spelling(TokenSpelling),
    Unresolved(WrittenOffset),
}
