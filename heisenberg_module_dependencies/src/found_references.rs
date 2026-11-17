use std::collections::BTreeMap;

use ra_ap_vfs::FileId;

use crate::found_reference::FoundReference;

#[derive(Default)]
pub struct FoundReferences {
    pub by_file: BTreeMap<FileId, Vec<FoundReference>>,
}
