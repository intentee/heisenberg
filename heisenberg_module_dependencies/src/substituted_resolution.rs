use std::rc::Rc;

use ra_ap_hir::GenericSubstitution;

#[derive(Clone)]
pub struct SubstitutedResolution<'database, TResolution> {
    pub resolution: TResolution,
    pub substitution: Option<Rc<GenericSubstitution<'database>>>,
}
