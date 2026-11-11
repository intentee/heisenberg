use ra_ap_hir::FileRangeWrapper;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::base_db::SourceDatabase;
use ra_ap_ide_db::line_index;
use ra_ap_ide_db::line_index::WideEncoding;
use ra_ap_ide_db::line_index::WideLineCol;
use ra_ap_paths::AbsPath;
use ra_ap_syntax::TextSize;
use ra_ap_vfs::FileId;

use heisenberg_module_graph::line_position::LinePosition;
use heisenberg_module_graph::source_location::SourceLocation;
use heisenberg_project_model::source_file::SourceFile;
use heisenberg_project_model::source_files::SourceFiles;

use crate::source_locator_error::SourceLocatorError;

pub struct SourceLocator<'project> {
    pub database: &'project RootDatabase,
    pub files: &'project SourceFiles,
}

impl SourceLocator<'_> {
    /// # Errors
    ///
    /// Returns [`SourceLocatorError::UnknownSourceFile`] when the loaded source files do not hold
    /// `file_id`.
    pub fn file<TError: From<SourceLocatorError>>(
        &self,
        file_id: FileId,
    ) -> Result<String, TError> {
        self.source_file(file_id)
            .map(|SourceFile { relative_path, .. }| relative_path.clone())
    }

    /// # Errors
    ///
    /// Returns [`SourceLocatorError`] when the loaded source files do not hold the file of the
    /// range, or when its line index does not hold the start of the range.
    pub fn locate<TError: From<SourceLocatorError>>(
        &self,
        FileRangeWrapper { file_id, range }: FileRangeWrapper<FileId>,
    ) -> Result<SourceLocation, TError> {
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
            .ok_or(SourceLocatorError::InvalidSourceOffset { file_id, offset })
            .map_err(TError::from)
            .and_then(|position| {
                self.file(file_id)
                    .map(|file| SourceLocation { file, position })
            })
    }

    /// # Errors
    ///
    /// Returns [`SourceLocatorError::UnknownSourceFile`] when the loaded source files do not hold
    /// `file_id`.
    pub fn path<TError: From<SourceLocatorError>>(
        &self,
        file_id: FileId,
    ) -> Result<&AbsPath, TError> {
        self.source_file(file_id)
            .map(|SourceFile { path, .. }| path.as_path())
    }

    /// # Errors
    ///
    /// Returns [`SourceLocatorError::UnknownSourceFile`] when the loaded source files do not hold
    /// `file_id`.
    pub fn source_file<TError: From<SourceLocatorError>>(
        &self,
        file_id: FileId,
    ) -> Result<&SourceFile, TError> {
        self.files
            .file(file_id)
            .ok_or(SourceLocatorError::UnknownSourceFile(file_id))
            .map_err(TError::from)
    }
}
