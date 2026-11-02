use ra_ap_hir::Const;
use ra_ap_hir::ExternCrateDecl;
use ra_ap_hir::Impl;
use ra_ap_hir::Module;
use ra_ap_hir::ModuleDef;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::ast;

use crate::item_adt::ItemAdt;

pub enum ItemDefinition {
    Const(Const),
    ExternBlock,
    ExternCrate(ExternCrateDecl),
    Impl(Impl),
    Module(Module),
    ModuleDef(ModuleDef),
    NotDefining,
    Unmapped,
}

impl ItemDefinition {
    pub fn new(item: &ast::Item, semantics: &Semantics<'_, RootDatabase>) -> Self {
        let module_def =
            |definition: Option<ModuleDef>| definition.map_or(Self::Unmapped, Self::ModuleDef);

        match item {
            ast::Item::AsmExpr(_) | ast::Item::MacroCall(_) | ast::Item::Use(_) => {
                Self::NotDefining
            }
            ast::Item::Const(definition) => semantics
                .to_def(definition)
                .map_or(Self::Unmapped, Self::Const),
            ast::Item::Enum(_) | ast::Item::Struct(_) | ast::Item::Union(_) => {
                module_def(ItemAdt { item, semantics }.adt().map(ModuleDef::Adt))
            }
            ast::Item::ExternBlock(definition) => semantics
                .to_def(definition)
                .map_or(Self::Unmapped, |_| Self::ExternBlock),
            ast::Item::ExternCrate(definition) => semantics
                .to_def(definition)
                .map_or(Self::Unmapped, Self::ExternCrate),
            ast::Item::Fn(definition) => {
                module_def(semantics.to_def(definition).map(ModuleDef::Function))
            }
            ast::Item::Impl(definition) => semantics
                .to_def(definition)
                .map_or(Self::Unmapped, Self::Impl),
            ast::Item::MacroDef(definition) => module_def(
                semantics
                    .to_macro_def(&ast::Macro::MacroDef(definition.clone()))
                    .map(ModuleDef::Macro),
            ),
            ast::Item::MacroRules(definition) => module_def(
                semantics
                    .to_macro_def(&ast::Macro::MacroRules(definition.clone()))
                    .map(ModuleDef::Macro),
            ),
            ast::Item::Module(definition) => semantics
                .to_def(definition)
                .map_or(Self::Unmapped, Self::Module),
            ast::Item::Static(definition) => {
                module_def(semantics.to_def(definition).map(ModuleDef::Static))
            }
            ast::Item::Trait(definition) => {
                module_def(semantics.to_def(definition).map(ModuleDef::Trait))
            }
            ast::Item::TypeAlias(definition) => {
                module_def(semantics.to_def(definition).map(ModuleDef::TypeAlias))
            }
        }
    }
}
