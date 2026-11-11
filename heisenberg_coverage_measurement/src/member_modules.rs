use std::collections::BTreeSet;

use ra_ap_vfs::FileId;

use heisenberg_member_crates::resolved_member_crate::ResolvedMemberCrate;

pub struct MemberModules<'modules> {
    pub members: &'modules [ResolvedMemberCrate],
    pub module_files: &'modules BTreeSet<FileId>,
}
