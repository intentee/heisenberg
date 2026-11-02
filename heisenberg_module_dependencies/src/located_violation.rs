use ra_ap_hir::FileRangeWrapper;
use ra_ap_vfs::FileId;

use heisenberg_module_graph::violation_kind::ViolationKind;

pub struct LocatedViolation {
    pub display: FileRangeWrapper<FileId>,
    pub kind: ViolationKind,
}
