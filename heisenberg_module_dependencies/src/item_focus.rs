use ra_ap_hir::FileRangeWrapper;
use ra_ap_hir::InFile;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::TextRange;
use ra_ap_vfs::FileId;

use heisenberg_source_locator::item_focus_range::item_focus_range;

pub struct ItemFocus<'focus, 'database> {
    pub node: &'focus SyntaxNode,
    pub semantics: &'focus Semantics<'database, RootDatabase>,
}

impl ItemFocus<'_, '_> {
    pub fn original_range(&self) -> FileRangeWrapper<FileId> {
        self.semantics
            .diagnostics_display_range_for_range(InFile::new(
                self.semantics.hir_file_for(self.node),
                self.range(),
            ))
    }

    pub fn range(&self) -> TextRange {
        item_focus_range(self.node)
    }

    pub fn text(&self) -> String {
        self.node.covering_element(self.range()).to_string()
    }
}
