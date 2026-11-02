use std::collections::BTreeSet;

use crate::inactive_focus::InactiveFocus;
use crate::written_tokens::WrittenTokens;

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct WalkedCode {
    pub active_tokens: WrittenTokens,
    pub files: BTreeSet<String>,
    pub inactive_foci: BTreeSet<InactiveFocus>,
    pub inactive_tokens: WrittenTokens,
}
