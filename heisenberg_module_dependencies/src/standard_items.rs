use ra_ap_hir::Adt;
use ra_ap_hir::Crate;
use ra_ap_hir::Enum;
use ra_ap_hir::ModuleDef;
use ra_ap_hir::Semantics;
use ra_ap_hir::Struct;
use ra_ap_hir::Trait;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::famous_defs::FamousDefs;

use heisenberg_module_graph::crate_target::CrateTarget;

use crate::constants::ERROR_TRAIT_PATH;
use crate::constants::OWNED_BOX_PATH;
use crate::constants::VECTOR_PATH;
use crate::module_dependencies_error::ModuleDependenciesError;
use crate::required_standard_item::RequiredStandardItem;
use crate::standard_item::standard_item;

fn missing<TItem>(
    item: Option<&TItem>,
    required: RequiredStandardItem,
) -> Option<RequiredStandardItem> {
    item.is_none().then_some(required)
}

pub struct StandardItems {
    pub display: Trait,
    pub error_trait: Trait,
    pub option: Enum,
    pub owned_box: Struct,
    pub result: Enum,
    pub to_string: Trait,
    pub vector: Struct,
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
        let error_trait =
            core.and_then(
                |core| match standard_item(database, core, &ERROR_TRAIT_PATH) {
                    Some(ModuleDef::Trait(error_trait)) => Some(error_trait),
                    _ => None,
                },
            );
        let display = famous.core_fmt_Display();
        let option = famous.core_option_Option();
        let owned_box = famous.alloc().and_then(|alloc| {
            match standard_item(database, alloc, &OWNED_BOX_PATH) {
                Some(ModuleDef::Adt(Adt::Struct(owned_box))) => Some(owned_box),
                _ => None,
            }
        });
        let result = famous.core_result_Result();
        let to_string = famous.alloc_string_ToString();
        let vector =
            famous
                .alloc()
                .and_then(|alloc| match standard_item(database, alloc, &VECTOR_PATH) {
                    Some(ModuleDef::Adt(Adt::Struct(vector))) => Some(vector),
                    _ => None,
                });

        if let Some(display) = display
            && let Some(error_trait) = error_trait
            && let Some(option) = option
            && let Some(owned_box) = owned_box
            && let Some(result) = result
            && let Some(to_string) = to_string
            && let Some(vector) = vector
        {
            Ok(Self {
                display,
                error_trait,
                option,
                owned_box,
                result,
                to_string,
                vector,
            })
        } else {
            Err(ModuleDependenciesError::StandardItemsMissing {
                crate_target: crate_target.clone(),
                items: [
                    missing(display.as_ref(), RequiredStandardItem::Display),
                    missing(error_trait.as_ref(), RequiredStandardItem::ErrorTrait),
                    missing(option.as_ref(), RequiredStandardItem::Option),
                    missing(owned_box.as_ref(), RequiredStandardItem::OwnedBox),
                    missing(result.as_ref(), RequiredStandardItem::Result),
                    missing(to_string.as_ref(), RequiredStandardItem::ToString),
                    missing(vector.as_ref(), RequiredStandardItem::Vector),
                ]
                .into_iter()
                .flatten()
                .collect(),
            })
        }
    }
}
