use ra_ap_hir::FileRangeWrapper;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::base_db::SourceDatabase;
use ra_ap_ide_db::line_index;
use ra_ap_ide_db::line_index::WideEncoding;
use ra_ap_ide_db::line_index::WideLineCol;
use ra_ap_syntax::TextSize;
use ra_ap_vfs::FileId;

use heisenberg_module_graph::line_position::LinePosition;
use heisenberg_module_graph::source_location::SourceLocation;
use heisenberg_project_model::source_file::SourceFile;
use heisenberg_project_model::source_files::SourceFiles;

use crate::module_dependencies_error::ModuleDependenciesError;

pub struct SourceLocator<'project> {
    pub database: &'project RootDatabase,
    pub files: &'project SourceFiles,
}

impl SourceLocator<'_> {
    pub fn file(&self, file_id: FileId) -> Result<String, ModuleDependenciesError> {
        self.files
            .file(file_id)
            .map(|SourceFile { relative_path, .. }| relative_path.clone())
            .ok_or(ModuleDependenciesError::UnknownSourceFile(file_id))
    }

    pub fn locate(
        &self,
        FileRangeWrapper { file_id, range }: FileRangeWrapper<FileId>,
    ) -> Result<SourceLocation, ModuleDependenciesError> {
        let text = self.database.file_text(file_id).text(self.database);
        let index = line_index(self.database, file_id);
        let offset = range.start();

        index
            .try_line_col(offset)
            .and_then(|line_col| {
                index.to_wide(WideEncoding::Utf32, line_col).and_then(
                    |WideLineCol { line, col }| {
                        offset
                            .checked_sub(TextSize::from(line_col.col))
                            .and_then(|line_start| text.get(usize::from(line_start)..))
                            .map(|from_line_start| LinePosition {
                                line: u64::from(line).saturating_add(1),
                                column: u64::from(col).saturating_add(1),
                                source_line: from_line_start
                                    .lines()
                                    .next()
                                    .unwrap_or_default()
                                    .to_owned(),
                            })
                    },
                )
            })
            .ok_or(ModuleDependenciesError::InvalidSourceOffset { file_id, offset })
            .and_then(|position| {
                self.file(file_id)
                    .map(|file| SourceLocation { file, position })
            })
    }
}
