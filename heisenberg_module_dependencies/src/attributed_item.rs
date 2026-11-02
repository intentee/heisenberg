use ra_ap_hir::Macro;
use ra_ap_hir::MacroKind;
use ra_ap_hir::ModuleDef;
use ra_ap_hir::PathResolution;
use ra_ap_hir::Semantics;
use ra_ap_hir_def::MacroId;
use ra_ap_hir_expand::MacroDefKind;
use ra_ap_hir_expand::builtin::BuiltinAttrExpander;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::ast;
use ra_ap_syntax::ast::HasAttrs;

pub struct AttributedItem<'item, 'database> {
    pub item: &'item ast::Item,
    pub semantics: &'item Semantics<'database, RootDatabase>,
}

impl AttributedItem<'_, '_> {
    pub fn has_procedural_attribute(&self) -> bool {
        self.attribute_macros()
            .into_iter()
            .any(|definition| definition.kind(self.semantics.db) == MacroKind::Attr)
    }

    pub fn has_test_attribute(&self) -> bool {
        self.attribute_macros().into_iter().any(|definition| {
            matches!(
                MacroId::from(definition).definition(self.semantics.db).kind,
                MacroDefKind::BuiltInAttr(
                    _,
                    BuiltinAttrExpander::Bench
                        | BuiltinAttrExpander::Test
                        | BuiltinAttrExpander::TestCase
                )
            )
        })
    }

    fn attribute_macros(&self) -> Vec<Macro> {
        self.item
            .attrs()
            .filter_map(|attribute| attribute.path())
            .filter_map(|path| match self.semantics.resolve_path(&path) {
                Some(PathResolution::Def(ModuleDef::Macro(definition))) => Some(definition),
                _ => None,
            })
            .collect()
    }
}
