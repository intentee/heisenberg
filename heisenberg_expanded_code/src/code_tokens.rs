use ra_ap_syntax::NodeOrToken;
use ra_ap_syntax::SyntaxElement;
use ra_ap_syntax::SyntaxToken;

pub fn code_tokens(
    elements: impl Iterator<Item = SyntaxElement>,
) -> impl Iterator<Item = SyntaxToken> {
    elements
        .filter_map(NodeOrToken::into_token)
        .filter(|token| !token.kind().is_trivia())
}
