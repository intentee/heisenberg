use ra_ap_hir::FileRangeWrapper;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::base_db::SourceDatabase;
use ra_ap_paths::AbsPathBuf;
use ra_ap_paths::Utf8Path;
use ra_ap_vfs::FileId;
use ra_ap_vfs::Vfs;

use heisenberg_module_graph::line_position::LinePosition;
use heisenberg_module_graph::source_location::SourceLocation;
use heisenberg_project_model::workspace_path::WorkspacePath;

pub struct SourceLocator<'project> {
    pub database: &'project RootDatabase,
    pub vfs: &'project Vfs,
    pub workspace_root: &'project AbsPathBuf,
}

impl SourceLocator<'_> {
    pub fn locate(
        &self,
        FileRangeWrapper { file_id, range }: FileRangeWrapper<FileId>,
    ) -> SourceLocation {
        let file_path = self.vfs.file_path(file_id).to_string();
        let text = self.database.file_text(file_id).text(self.database);
        let start = usize::from(range.start());
        let mut line: u32 = 1;
        let mut column: u32 = 1;

        for (_, character) in text
            .char_indices()
            .take_while(|(offset, _)| *offset < start)
        {
            if character == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }

        SourceLocation {
            file: WorkspacePath {
                root: AsRef::<Utf8Path>::as_ref(self.workspace_root),
            }
            .relative(Utf8Path::new(&file_path)),
            position: LinePosition {
                column,
                line,
                source_line: text
                    .lines()
                    .nth(line as usize - 1)
                    .unwrap_or_default()
                    .to_owned(),
            },
        }
    }
}
