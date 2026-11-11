use ra_ap_hir::InFile;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::TextRange;

pub struct CoveringAncestors<'ancestors, 'database> {
    pub range: InFile<TextRange>,
    pub semantics: &'ancestors Semantics<'database, RootDatabase>,
}

impl CoveringAncestors<'_, '_> {
    pub fn nodes(&self) -> impl Iterator<Item = SyntaxNode> {
        let InFile { file_id, value } = self.range;

        self.semantics
            .parse_or_expand(file_id)
            .covering_element(value)
            .ancestors()
    }
}
