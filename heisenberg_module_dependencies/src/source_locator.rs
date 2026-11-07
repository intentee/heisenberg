use std::cell::RefCell;
use std::collections::HashMap;

use ra_ap_hir::FileRangeWrapper;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::base_db::SourceDatabase;
use ra_ap_ide_db::line_index;
use ra_ap_ide_db::line_index::LineCol;
use ra_ap_paths::AbsPathBuf;
use ra_ap_paths::Utf8Path;
use ra_ap_syntax::TextRange;
use ra_ap_syntax::TextSize;
use ra_ap_vfs::FileId;

use heisenberg_module_graph::line_position::LinePosition;
use heisenberg_module_graph::source_location::SourceLocation;
use heisenberg_project_model::source_files::SourceFiles;
use heisenberg_project_model::workspace_path::WorkspacePath;

pub struct SourceLocator<'project> {
    pub database: &'project RootDatabase,
    pub files: &'project SourceFiles,
    pub relative_paths: &'project RefCell<HashMap<FileId, String>>,
    pub workspace_root: &'project AbsPathBuf,
}

impl SourceLocator<'_> {
    pub fn file(&self, file_id: FileId) -> String {
        self.relative_paths
            .borrow_mut()
            .entry(file_id)
            .or_insert_with(|| {
                WorkspacePath {
                    root: AsRef::<Utf8Path>::as_ref(self.workspace_root),
                }
                .relative(self.files.path(file_id).as_ref())
            })
            .clone()
    }

    pub fn locate(
        &self,
        FileRangeWrapper { file_id, range }: FileRangeWrapper<FileId>,
    ) -> SourceLocation {
        let text = self.database.file_text(file_id).text(self.database);
        let offset = range.start();
        let LineCol { line, col } = line_index(self.database, file_id).line_col(offset);
        let line_start = offset - TextSize::from(col);

        SourceLocation {
            file: self.file(file_id),
            position: LinePosition {
                column: text[TextRange::new(line_start, offset)]
                    .chars()
                    .fold(1, |column, _| column + 1),
                line: line + 1,
                source_line: text[usize::from(line_start)..]
                    .lines()
                    .next()
                    .unwrap_or_default()
                    .to_owned(),
            },
        }
    }
}
