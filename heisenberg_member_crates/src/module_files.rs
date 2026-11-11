use std::collections::BTreeSet;

use ra_ap_vfs::FileId;

use crate::resolved_member_crate::ResolvedMemberCrate;

pub fn module_files<'members>(
    members: impl Iterator<Item = &'members ResolvedMemberCrate>,
) -> BTreeSet<FileId> {
    members
        .flat_map(|ResolvedMemberCrate { files, .. }| files.iter().copied())
        .collect()
}
