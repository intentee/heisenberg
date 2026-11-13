use ra_ap_hir::Adt;
use ra_ap_hir::Crate;
use ra_ap_hir::Enum;
use ra_ap_hir::Function;
use ra_ap_hir::ModuleDef;
use ra_ap_hir::Semantics;
use ra_ap_hir::Trait;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::famous_defs::FamousDefs;

use crate::constants::ERROR_TRAIT_PATH;
use crate::constants::INFALLIBLE_PATH;
use crate::constants::STANDARD_SLEEP_FUNCTIONS;
use crate::module_dependencies_error::ModuleDependenciesError;
use crate::standard_item::standard_item;

fn missing<TItem>(item: Option<&TItem>, name: &'static str) -> Option<&'static str> {
    item.is_none().then_some(name)
}

pub struct StandardItems {
    pub drop: Function,
    pub error_trait: Trait,
    pub from_str: Trait,
    pub infallible: Enum,
    pub into_iterator: Trait,
    pub iterator: Trait,
    pub library_crates: Vec<Crate>,
    pub result: Enum,
    pub sleeps: Vec<Function>,
    pub to_string: Trait,
}

impl StandardItems {
    pub fn new(
        semantics: &Semantics<'_, RootDatabase>,
        krate: Crate,
    ) -> Result<Self, ModuleDependenciesError> {
        let database = semantics.db;
        let famous = FamousDefs(semantics, krate);
        let core = famous.core();
        let drop = famous.core_mem_drop();
        let error_trait =
            core.and_then(
                |core| match standard_item(database, core, &ERROR_TRAIT_PATH) {
                    Some(ModuleDef::Trait(error_trait)) => Some(error_trait),
                    _ => None,
                },
            );
        let infallible =
            core.and_then(
                |core| match standard_item(database, core, &INFALLIBLE_PATH) {
                    Some(ModuleDef::Adt(Adt::Enum(infallible))) => Some(infallible),
                    _ => None,
                },
            );
        let from_str = famous.core_convert_FromStr();
        let into_iterator = famous.core_iter_IntoIterator();
        let iterator = famous.core_iter_Iterator();
        let result = famous.core_result_Result();
        let to_string = famous.alloc_string_ToString();

        if let Some(drop) = drop
            && let Some(error_trait) = error_trait
            && let Some(from_str) = from_str
            && let Some(infallible) = infallible
            && let Some(into_iterator) = into_iterator
            && let Some(iterator) = iterator
            && let Some(result) = result
            && let Some(to_string) = to_string
        {
            Ok(Self {
                drop,
                error_trait,
                from_str,
                infallible,
                into_iterator,
                iterator,
                library_crates: core
                    .into_iter()
                    .chain(famous.alloc())
                    .chain(famous.std())
                    .collect(),
                result,
                sleeps: famous
                    .std()
                    .into_iter()
                    .flat_map(|std| {
                        STANDARD_SLEEP_FUNCTIONS
                            .iter()
                            .filter_map(move |path| match standard_item(database, std, path) {
                                Some(ModuleDef::Function(function)) => Some(function),
                                _ => None,
                            })
                    })
                    .collect(),
                to_string,
            })
        } else {
            Err(ModuleDependenciesError::StandardItemsMissing {
                items: [
                    missing(drop.as_ref(), "core::mem::drop"),
                    missing(error_trait.as_ref(), "core::error::Error"),
                    missing(from_str.as_ref(), "core::str::FromStr"),
                    missing(infallible.as_ref(), "core::convert::Infallible"),
                    missing(into_iterator.as_ref(), "core::iter::IntoIterator"),
                    missing(iterator.as_ref(), "core::iter::Iterator"),
                    missing(result.as_ref(), "core::result::Result"),
                    missing(to_string.as_ref(), "alloc::string::ToString"),
                ]
                .into_iter()
                .flatten()
                .collect(),
            })
        }
    }
}
