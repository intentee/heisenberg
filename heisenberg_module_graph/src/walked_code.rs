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

impl WalkedCode {
    pub fn merge(
        &mut self,
        Self {
            active_tokens,
            files,
            inactive_foci,
            inactive_tokens,
        }: Self,
    ) {
        self.active_tokens.merge(active_tokens);
        self.files.extend(files);
        self.inactive_foci.extend(inactive_foci);
        self.inactive_tokens.merge(inactive_tokens);
    }
}
