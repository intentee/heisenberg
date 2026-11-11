use std::collections::BTreeSet;

use ra_ap_hir::InFile;
use ra_ap_ide_db::FilePosition;
use ra_ap_syntax::TextRange;
use ra_ap_vfs::FileId;

use heisenberg_configured_code::test_modules::TestModules;
use heisenberg_task_graph::once_map::OnceMap;

use crate::active_code::ActiveCode;

pub struct MeasurementRun {
    pub active_code: ActiveCode,
    pub measured_files: BTreeSet<FileId>,
    pub measured_positions: OnceMap<InFile<TextRange>, Option<FilePosition>>,
    pub test_modules: TestModules,
}
