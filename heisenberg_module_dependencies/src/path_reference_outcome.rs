use ra_ap_hir::Module;

use crate::referenced_modules::ReferencedModules;

pub enum PathReferenceOutcome {
    Dependency(ReferencedModules),
    Irrelevant,
    Unresolved,
    UnresolvedInForeignModule { crate_name: String, module: Module },
}
