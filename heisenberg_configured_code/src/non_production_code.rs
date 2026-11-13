use ra_ap_syntax::SyntaxNode;
use ra_ap_vfs::FileId;

use heisenberg_source_locator::original_file_id::original_file_id;

use crate::non_production_files::NonProductionFiles;
use crate::test_module_code::TestModuleCode;

pub struct NonProductionCode<'code, 'database> {
    pub files: &'code NonProductionFiles,
    pub modules: TestModuleCode<'code, 'database>,
}

impl NonProductionCode<'_, '_> {
    #[must_use]
    pub fn contains(&self, node: &SyntaxNode) -> bool {
        let file = self.original_file(node);

        self.files.benchmark.contains(&file)
            || self.files.test.contains(&file)
            || self.modules.contains(node)
    }

    #[must_use]
    pub fn is_benchmark(&self, node: &SyntaxNode) -> bool {
        self.files.benchmark.contains(&self.original_file(node))
    }

    fn original_file(&self, node: &SyntaxNode) -> FileId {
        let semantics = self.modules.semantics;

        original_file_id(semantics.db, semantics.hir_file_for(node))
    }
}
