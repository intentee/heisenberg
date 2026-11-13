use std::collections::HashSet;

use ra_ap_hir::AsAssocItem as _;
use ra_ap_hir::AssocItemContainer;
use ra_ap_hir::Crate;
use ra_ap_hir::Function;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;

use heisenberg_project_model::constants::VETTED_BENCHMARK_HARNESS_METHODS;
use heisenberg_project_model::harness_methods::HarnessMethods;

use crate::harness_method::HarnessMethod;

pub struct BenchmarkHarness<'harness, 'database> {
    pub crates: &'harness HashSet<Crate>,
    pub semantics: &'harness Semantics<'database, RootDatabase>,
}

impl BenchmarkHarness<'_, '_> {
    pub fn method(&self, function: Function) -> Option<HarnessMethod> {
        let database = self.semantics.db;

        (!self.crates.is_empty())
            .then(|| function.as_assoc_item(database))
            .flatten()
            .and_then(|item| match item.container(database) {
                AssocItemContainer::Impl(implementation) => Some(implementation),
                AssocItemContainer::Trait(_) => None,
            })
            .filter(|implementation| implementation.trait_(database).is_none())
            .and_then(|implementation| implementation.self_ty(database).as_adt())
            .filter(|adt| self.crates.contains(&adt.module(database).krate(database)))
            .and_then(|adt| {
                let adt_name = adt.name(database);

                VETTED_BENCHMARK_HARNESS_METHODS
                    .iter()
                    .map(|HarnessMethods { type_name, .. }| *type_name)
                    .find(|type_name| adt_name.as_str() == *type_name)
            })
            .map(|type_name| HarnessMethod {
                name: function.name(database).as_str().to_owned(),
                type_name,
            })
    }
}
