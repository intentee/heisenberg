use std::iter::successors;

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
    pub fn nodes(&self) -> Vec<SyntaxNode> {
        successors(
            Some(self.ancestors_with_macros(self.node.clone())),
            |segment: &Vec<SyntaxNode>| {
                segment
                    .last()
                    .and_then(|root| self.declaration(root))
                    .map(|declaration| self.ancestors_with_macros(declaration))
            },
        )
        .flatten()
        .collect()
    }

    fn ancestors_with_macros(&self, node: SyntaxNode) -> Vec<SyntaxNode> {
        self.semantics.ancestors_with_macros(node).collect()
    }

    fn declaration(&self, root: &SyntaxNode) -> Option<SyntaxNode> {
        self.semantics
            .scope(root)
            .and_then(|scope| scope.module().declaration_source(self.semantics.db))
            .map(|InFile { file_id, value }| {
                SyntaxNodePtr::new(value.syntax()).to_node(&self.semantics.parse_or_expand(file_id))
            })
    }
}
