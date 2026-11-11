use std::collections::BTreeSet;

use ra_ap_syntax::SyntaxNode;
use ra_ap_vfs::FileId;

use heisenberg_source_locator::original_file_id::original_file_id;

use crate::test_module_code::TestModuleCode;

pub struct TestCode<'code, 'database> {
    pub files: &'code BTreeSet<FileId>,
    pub modules: TestModuleCode<'code, 'database>,
}

impl TestCode<'_, '_> {
    #[must_use]
    pub fn contains(&self, node: &SyntaxNode) -> bool {
        let semantics = self.modules.semantics;

        self.files.contains(&original_file_id(
            semantics.db,
            semantics.hir_file_for(node),
        )) || self.modules.contains(node)
    }
}
