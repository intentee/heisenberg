use ra_ap_hir::Crate;
use ra_ap_ide_db::RootDatabase;

use heisenberg_module_graph::crate_target::CrateTarget;

use crate::crate_module_files::crate_module_files;
use crate::resolved_member_crate::ResolvedMemberCrate;

pub struct MemberCrate {
    pub crate_target: CrateTarget,
    pub krate: Crate,
}

impl MemberCrate {
    pub fn resolved(self, database: &RootDatabase) -> ResolvedMemberCrate {
        ResolvedMemberCrate {
            files: crate_module_files(database, self.krate),
            crate_target: self.crate_target,
            krate: self.krate,
        }
    }
}
