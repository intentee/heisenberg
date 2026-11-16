use ra_ap_hir::Crate;
use ra_ap_hir::Enum;
use ra_ap_hir::Function;
use ra_ap_hir::ModuleDef;
use ra_ap_hir::Semantics;
use ra_ap_hir::Trait;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::famous_defs::FamousDefs;

use heisenberg_module_graph::crate_target::CrateTarget;

use crate::constants::ERROR_TRAIT_PATH;
use crate::constants::STANDARD_SLEEP_MS_PATH;
use crate::constants::STANDARD_SLEEP_PATH;
use crate::module_dependencies_error::ModuleDependenciesError;
use crate::required_standard_item::RequiredStandardItem;
use crate::standard_item::standard_item;

fn missing<TItem>(
    item: Option<&TItem>,
    required: RequiredStandardItem,
) -> Option<RequiredStandardItem> {
    item.is_none().then_some(required)
}

fn standard_function(database: &RootDatabase, krate: Crate, path: &[&str]) -> Option<Function> {
    match standard_item(database, krate, path) {
        Some(ModuleDef::Function(function)) => Some(function),
        _ => None,
    }
}

pub struct StandardItems {
    pub error_trait: Trait,
    pub from_str: Trait,
    pub library_crates: Vec<Crate>,
    pub result: Enum,
    pub sleeps: Vec<Function>,
    pub to_string: Trait,
}

impl StandardItems {
    pub fn new(
        semantics: &Semantics<'_, RootDatabase>,
        krate: Crate,
        crate_target: &CrateTarget,
    ) -> Result<Self, ModuleDependenciesError> {
        let database = semantics.db;
        let famous = FamousDefs(semantics, krate);
        let core = famous.core();
        let std = famous.std();
        let error_trait =
            core.and_then(
                |core| match standard_item(database, core, &ERROR_TRAIT_PATH) {
                    Some(ModuleDef::Trait(error_trait)) => Some(error_trait),
                    _ => None,
                },
            );
        let from_str = famous.core_convert_FromStr();
        let result = famous.core_result_Result();
        let to_string = famous.alloc_string_ToString();
        let sleep = std.and_then(|std| standard_function(database, std, &STANDARD_SLEEP_PATH));
        let sleep_ms =
            std.and_then(|std| standard_function(database, std, &STANDARD_SLEEP_MS_PATH));
        let missing_sleeps: Vec<RequiredStandardItem> = std
            .into_iter()
            .flat_map(|_| {
                [
                    missing(sleep.as_ref(), RequiredStandardItem::Sleep),
                    missing(sleep_ms.as_ref(), RequiredStandardItem::SleepMs),
                ]
            })
            .flatten()
            .collect();

        if let Some(error_trait) = error_trait
            && let Some(from_str) = from_str
            && let Some(result) = result
            && let Some(to_string) = to_string
            && missing_sleeps.is_empty()
        {
            Ok(Self {
                error_trait,
                from_str,
                library_crates: core.into_iter().chain(famous.alloc()).chain(std).collect(),
                result,
                sleeps: sleep.into_iter().chain(sleep_ms).collect(),
                to_string,
            })
        } else {
            Err(ModuleDependenciesError::StandardItemsMissing {
                crate_target: crate_target.clone(),
                items: [
                    missing(error_trait.as_ref(), RequiredStandardItem::ErrorTrait),
                    missing(from_str.as_ref(), RequiredStandardItem::FromStr),
                    missing(result.as_ref(), RequiredStandardItem::Result),
                    missing(to_string.as_ref(), RequiredStandardItem::ToString),
                ]
                .into_iter()
                .flatten()
                .chain(missing_sleeps)
                .collect(),
            })
        }
    }
}
