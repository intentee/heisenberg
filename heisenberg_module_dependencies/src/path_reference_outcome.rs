use crate::referenced_modules::ReferencedModules;

pub enum PathReferenceOutcome {
    Dependency(ReferencedModules),
    Irrelevant,
    Unresolved,
}
