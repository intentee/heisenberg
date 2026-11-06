use ra_ap_hir::HasVisibility;
use ra_ap_hir::ModuleDef;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::ast;

use crate::item_definition::ItemDefinition;
use crate::outside_visibility::OutsideVisibility;

pub struct ItemVisibility<'item, 'database> {
    pub item: &'item ast::Item,
    pub semantics: &'item Semantics<'database, RootDatabase>,
}

impl ItemVisibility<'_, '_> {
    pub fn is_public(&self) -> bool {
        let database = self.semantics.db;
        let outside = OutsideVisibility { database };

        match ItemDefinition::new(self.item, self.semantics) {
            ItemDefinition::Const(constant) => {
                outside.definition_reaches_outside(ModuleDef::Const(constant))
            }
            ItemDefinition::ModuleDef(definition) => {
                if matches!(self.item, ast::Item::MacroRules(_)) {
                    definition.visibility(database).is_public()
                } else {
                    outside.definition_reaches_outside(definition)
                }
            }
            ItemDefinition::ExternBlock
            | ItemDefinition::ExternCrate(_)
            | ItemDefinition::Impl(_)
            | ItemDefinition::Module(_)
            | ItemDefinition::NotDefining
            | ItemDefinition::Unmapped => false,
        }
    }
}
