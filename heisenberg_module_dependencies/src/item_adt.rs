use ra_ap_hir::Adt;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::ast;

pub struct ItemAdt<'item, 'database> {
    pub item: &'item ast::Item,
    pub semantics: &'item Semantics<'database, RootDatabase>,
}

impl ItemAdt<'_, '_> {
    pub fn adt(&self) -> Option<Adt> {
        ast::Adt::cast(self.item.syntax().clone())
            .and_then(|definition| self.semantics.to_def(&definition))
    }
}
