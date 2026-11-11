use ra_ap_hir::MacroCallId;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::NodeOrToken;
use ra_ap_syntax::Parse;
use ra_ap_syntax::SyntaxError;
use ra_ap_syntax::SyntaxKind;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::SyntaxToken;

use heisenberg_expanded_code::expansion_syntax_errors::expansion_syntax_errors;

pub struct MacroExpansion<'expansion> {
    pub call: MacroCallId,
    pub database: &'expansion RootDatabase,
}

impl<'expansion> MacroExpansion<'expansion> {
    pub fn syntax_errors(&self) -> &'expansion [SyntaxError] {
        expansion_syntax_errors(self.database, self.call)
    }

    pub fn unreadable_literals(&self) -> Vec<SyntaxToken> {
        self.parse()
            .syntax_node()
            .descendants_with_tokens()
            .filter_map(NodeOrToken::into_token)
            .filter(|token| token.kind() == SyntaxKind::ERROR)
            .collect()
    }

    fn parse(&self) -> Parse<SyntaxNode> {
        let (parse, _) = &self.call.parse_macro_expansion(self.database).value;

        parse.clone()
    }
}
