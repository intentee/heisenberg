use std::collections::BTreeSet;

use ra_ap_hir::HirFileId;
use ra_ap_hir::InFile;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::FilePosition;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxToken;
use ra_ap_syntax::TextRange;
use ra_ap_syntax::ast;
use ra_ap_vfs::FileId;

use heisenberg_expanded_code::generated_tokens::GeneratedTokens;
use heisenberg_source_locator::written_range::written_range;
use heisenberg_task_graph::once_map::OnceMap;

use crate::coverage_measurement_error::CoverageMeasurementError;

pub struct FunctionIdentity<'identity, 'database> {
    pub measured_files: &'identity BTreeSet<FileId>,
    pub measured_positions: &'identity OnceMap<InFile<TextRange>, Option<FilePosition>>,
    pub semantics: &'identity Semantics<'database, RootDatabase>,
}

impl FunctionIdentity<'_, '_> {
    pub fn body_brace(
        &self,
        file_id: HirFileId,
        function: &ast::Fn,
    ) -> Result<Option<FilePosition>, CoverageMeasurementError> {
        function.body().map_or(Ok(None), |body| {
            let offset = body.syntax().text_range().start();

            body.stmt_list()
                .and_then(|statements| statements.l_curly_token())
                .ok_or(CoverageMeasurementError::FunctionBodyWithoutBraces { file_id, offset })
                .map(|brace| self.measured_position(file_id, &brace))
        })
    }

    pub fn measured_position(
        &self,
        file_id: HirFileId,
        token: &SyntaxToken,
    ) -> Option<FilePosition> {
        let range = token.text_range();

        self.measured_positions.get_or_init(
            InFile {
                file_id,
                value: range,
            },
            || {
                let written = written_range(self.semantics.db, file_id, range);

                GeneratedTokens {
                    semantics: self.semantics,
                }
                .is_written_in(file_id, range, &written, self.measured_files)
                .then_some(FilePosition {
                    file_id: written.file_id,
                    offset: written.range.start(),
                })
            },
        )
    }
}
