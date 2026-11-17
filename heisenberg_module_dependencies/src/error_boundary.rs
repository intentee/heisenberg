use ra_ap_hir::Adt;
use ra_ap_hir::Crate;
use ra_ap_hir::EnumVariant;
use ra_ap_hir::Type;

use heisenberg_module_graph::error_violation::ErrorViolation;
use heisenberg_module_graph::untyped_error_name::UntypedErrorName;

use crate::error_types::ErrorTypes;
use crate::token_origin::TokenOrigin;

pub struct ErrorBoundary<'boundary, 'database> {
    pub error_types: ErrorTypes<'boundary, 'database>,
    pub krate: Crate,
    pub token_origin: &'boundary TokenOrigin<'boundary, 'database>,
}

impl<'database> ErrorBoundary<'_, 'database> {
    pub fn outside_owner(&self, candidate: &Type<'database>) -> Option<Crate> {
        let database = self.error_types.semantics.db;
        let carried = self.carried(candidate);

        self.error_types
            .implements_error(&carried)
            .then(|| carried.as_adt())
            .flatten()
            .map(|adt| adt.module(database).krate(database))
            .filter(|owner| *owner != self.krate && self.token_origin.is_workspace_crate(*owner))
    }

    pub fn outside_owner_of_variant(&self, variant: EnumVariant) -> Option<Crate> {
        let database = self.error_types.semantics.db;

        self.outside_owner(&variant.parent_enum(database).ty(database))
    }

    pub fn untyped_violation(
        &self,
        error: &Type<'database>,
        name: UntypedErrorName,
    ) -> ErrorViolation {
        let database = self.error_types.semantics.db;

        if error.as_adt().is_some_and(|adt| {
            self.token_origin
                .is_workspace_crate(adt.module(database).krate(database))
        }) {
            ErrorViolation::NotAnError { error: name }
        } else {
            ErrorViolation::UntypedResult { error: name }
        }
    }

    fn carried(&self, candidate: &Type<'database>) -> Type<'database> {
        candidate
            .type_arguments()
            .next()
            .filter(|_| {
                matches!(
                    candidate.as_adt(),
                    Some(Adt::Struct(wrapper))
                        if wrapper == self.error_types.standard_items.owned_box
                            || wrapper == self.error_types.standard_items.vector
                )
            })
            .map_or_else(|| candidate.clone(), |wrapped| self.carried(&wrapped))
    }
}
