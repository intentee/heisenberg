use ra_ap_hir::FileRangeWrapper;
use ra_ap_hir::InFile;
use ra_ap_syntax::TextRange;
use ra_ap_vfs::FileId;

use heisenberg_module_graph::violation_kind::ViolationKind;

pub struct DiagnosticSite {
    pub display: FileRangeWrapper<FileId>,
    pub kind: ViolationKind,
    pub origin: InFile<TextRange>,
}
