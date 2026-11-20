use ra_ap_syntax::AstNode;
use ra_ap_syntax::NodeOrToken;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::TextRange;
use ra_ap_syntax::ast;
use ra_ap_syntax::ast::HasName;

#[must_use]
pub fn item_focus_range(node: &SyntaxNode) -> TextRange {
    let whole = node.text_range();

    ast::AnyHasName::cast(node.clone())
        .and_then(|named| named.name())
        .map(|name| name.syntax().text_range())
        .or_else(|| {
            ast::Impl::cast(node.clone())
                .and_then(|implementation| implementation.self_ty())
                .map(|self_type| self_type.syntax().text_range())
        })
        .or_else(|| {
            ast::Use::cast(node.clone())
                .and_then(|import| import.use_tree())
                .map(|tree| tree.syntax().text_range())
        })
        .or_else(|| {
            ast::ExternCrate::cast(node.clone())
                .and_then(|declaration| declaration.name_ref())
                .map(|name| name.syntax().text_range())
        })
        .or_else(|| ast::SourceFile::can_cast(node.kind()).then_some(whole))
        .or_else(|| {
            node.children_with_tokens()
                .find(|element| match element {
                    NodeOrToken::Node(child) => {
                        !ast::Attr::can_cast(child.kind())
                            && !ast::Visibility::can_cast(child.kind())
                    }
                    NodeOrToken::Token(token) => !token.kind().is_trivia(),
                })
                .map(|element| element.text_range())
        })
        .unwrap_or(whole)
}
