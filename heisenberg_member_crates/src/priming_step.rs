use ra_ap_hir::Crate;
use ra_ap_hir::TraitImpls;
use ra_ap_hir::attach_db;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::base_db::Crate as DatabaseCrate;

pub enum PrimingStep {
    NameResolution(Crate),
    TraitImplementations(Crate),
}

impl PrimingStep {
    pub fn prime(&self, database: &RootDatabase) {
        match self {
            Self::NameResolution(krate) => {
                krate.root_module(database);
            }
            Self::TraitImplementations(krate) => attach_db(database, || {
                TraitImpls::for_crate(database, DatabaseCrate::from(*krate));
            }),
        }
    }
}
