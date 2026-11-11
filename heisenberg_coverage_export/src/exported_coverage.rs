use std::collections::BTreeMap;
use std::time::SystemTime;

use ra_ap_paths::AbsPathBuf;

use crate::exported_region::ExportedRegion;

pub struct ExportedCoverage {
    pub files: BTreeMap<AbsPathBuf, Vec<ExportedRegion>>,
    pub modified: SystemTime,
    pub path: AbsPathBuf,
}
