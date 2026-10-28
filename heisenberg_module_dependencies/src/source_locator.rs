use heisenberg_module_graph::source_location::SourceLocation;
use ra_ap_hir::FileRangeWrapper;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::base_db::SourceDatabase;
use ra_ap_paths::AbsPathBuf;
use ra_ap_vfs::FileId;
use ra_ap_vfs::Vfs;

pub struct SourceLocator<'project> {
    pub database: &'project RootDatabase,
    pub vfs: &'project Vfs,
    pub workspace_root: &'project AbsPathBuf,
}

impl SourceLocator<'_> {
    #[expect(
        clippy::expect_used,
        reason = "rust-analyzer loads every source file from a real path inside a member package, and every member lies inside the workspace root"
    )]
    pub fn locate(&self, range: FileRangeWrapper<FileId>) -> SourceLocation {
        let absolute = self
            .vfs
            .file_path(range.file_id)
            .as_path()
            .expect("rust-analyzer loaded the source file from a real path");
        let text = self.database.file_text(range.file_id).text(self.database);
        let prefix = &text[..usize::from(range.range.start())];

        SourceLocation {
            column: prefix
                .chars()
                .rev()
                .take_while(|character| *character != '\n')
                .count() as u32
                + 1,
            file: absolute
                .strip_prefix(self.workspace_root)
                .expect("every analysed source file lies inside the workspace root")
                .as_utf8_path()
                .components()
                .map(|component| component.as_str())
                .collect::<Vec<&str>>()
                .join("/"),
            line: prefix.bytes().filter(|byte| *byte == b'\n').count() as u32 + 1,
        }
    }
}
