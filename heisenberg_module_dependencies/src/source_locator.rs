use std::path::Path;

use ra_ap_hir::FileRangeWrapper;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::base_db::SourceDatabase;
use ra_ap_paths::AbsPathBuf;
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
    #[expect(
        clippy::expect_used,
        reason = "rust-analyzer loads every source file from a real path"
    )]
    pub fn locate(
        &self,
        FileRangeWrapper { file_id, range }: FileRangeWrapper<FileId>,
    ) -> SourceLocation {
        let absolute = self
            .vfs
            .file_path(file_id)
            .as_path()
            .expect("rust-analyzer loaded the source file from a real path");
        let text = self.database.file_text(file_id).text(self.database);
        let prefix = &text[..usize::from(range.start())];
        let line_start = prefix.rfind('\n').map_or(0, |newline| newline + 1);

        SourceLocation {
            file: WorkspacePath {
                root: self.workspace_root.as_ref(),
            }
            .relative(AsRef::<Path>::as_ref(absolute)),
            position: LinePosition {
                column: prefix
                    .chars()
                    .rev()
                    .take_while(|character| *character != '\n')
                    .count() as u32
                    + 1,
                line: prefix.bytes().filter(|byte| *byte == b'\n').count() as u32 + 1,
                source_line: text[line_start..]
                    .lines()
                    .next()
                    .unwrap_or_default()
                    .to_owned(),
            },
        }
    }
}
