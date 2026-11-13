use std::collections::BTreeSet;

use ra_ap_vfs::FileId;

pub struct NonProductionFiles {
    pub benchmark: BTreeSet<FileId>,
    pub test: BTreeSet<FileId>,
}
