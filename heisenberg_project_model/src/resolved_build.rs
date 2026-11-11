use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::build_link::BuildLink;

pub struct ResolvedBuild {
    pub features: BTreeMap<String, BTreeSet<String>>,
    pub links: BTreeSet<BuildLink>,
}
