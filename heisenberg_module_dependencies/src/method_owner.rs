use ra_ap_hir::Impl;
use ra_ap_hir::Trait;

#[derive(Clone, Copy)]
pub enum MethodOwner {
    InherentImpl(Impl),
    TraitDeclaration(Trait),
}
