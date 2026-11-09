use std::num::NonZeroU16;

use cargo::util::data_structures::HashMap;
use ra_ap_paths::AbsPathBuf;

pub struct RunContext<'model> {
    pub cargo_home: &'model AbsPathBuf,
    pub environment: HashMap<String, String>,
    pub jobs: NonZeroU16,
    pub sysroot: &'model AbsPathBuf,
    pub workspace_directory: &'model AbsPathBuf,
}
