use ra_ap_hir::InFile;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::SyntaxNodePtr;

pub struct Ancestry<'ancestry, 'database> {
    pub node: &'ancestry SyntaxNode,
    pub semantics: &'ancestry Semantics<'database, RootDatabase>,
}

impl Ancestry<'_, '_> {
    #[must_use]
    pub fn enclosing(&self) -> Option<SyntaxNode> {
        self.semantics
            .ancestors_with_macros(self.node.clone())
            .nth(1)
            .or_else(|| self.declaration())
    }

    fn declaration(&self) -> Option<SyntaxNode> {
        self.semantics
            .scope(self.node)
            .and_then(|scope| scope.module().declaration_source(self.semantics.db))
            .map(|InFile { file_id, value }| {
                SyntaxNodePtr::new(value.syntax()).to_node(&self.semantics.parse_or_expand(file_id))
            })
    }
}
