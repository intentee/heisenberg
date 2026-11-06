use ra_ap_syntax::AstNode;
use ra_ap_syntax::ast;

pub struct SignatureTuple<'tuple> {
    pub tuple: &'tuple ast::TupleType,
}

impl SignatureTuple<'_> {
    pub fn is_in_a_signature(&self) -> bool {
        self.tuple.fields().nth(1).is_some()
            && self
                .tuple
                .syntax()
                .ancestors()
                .find(|ancestor| {
                    ast::Item::can_cast(ancestor.kind())
                        || ast::Expr::can_cast(ancestor.kind())
                        || ast::LetStmt::can_cast(ancestor.kind())
                })
                .and_then(ast::Item::cast)
                .is_some_and(|item| !Self::is_dictated_by_a_trait(&item))
    }

    fn is_dictated_by_a_trait(item: &ast::Item) -> bool {
        !matches!(item, ast::Item::TypeAlias(_))
            && item
                .syntax()
                .parent()
                .and_then(ast::AssocItemList::cast)
                .and_then(|list| list.syntax().parent())
                .and_then(ast::Impl::cast)
                .is_some_and(|implementation| implementation.trait_().is_some())
    }
}
