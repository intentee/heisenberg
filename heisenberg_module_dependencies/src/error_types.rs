use ra_ap_hir::Adt;
use ra_ap_hir::Semantics;
use ra_ap_hir::Type;
use ra_ap_ide_db::RootDatabase;

use crate::standard_items::StandardItems;

pub struct ErrorTypes<'types, 'database> {
    pub semantics: &'types Semantics<'database, RootDatabase>,
    pub standard_items: &'types StandardItems,
}

impl<'database> ErrorTypes<'_, 'database> {
    pub fn implements_error(&self, candidate: &Type<'database>) -> bool {
        candidate.impls_trait(self.semantics.db, self.standard_items.error_trait, &[])
    }

    pub fn is_error_type(&self, candidate: &Type<'database>) -> bool {
        candidate.is_unknown() || self.implements_error(candidate)
    }

    pub fn result_error(&self, candidate: &Type<'database>) -> Option<Type<'database>> {
        (candidate.as_adt() == Some(Adt::Enum(self.standard_items.result)))
            .then(|| candidate.type_arguments().nth(1))
            .flatten()
    }
}
