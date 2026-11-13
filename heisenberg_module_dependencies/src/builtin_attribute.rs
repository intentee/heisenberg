use ra_ap_hir::PathResolution;
use ra_ap_intern::Symbol;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::ast;

use crate::path_resolutions::PathResolutions;

pub struct BuiltinAttribute<'attribute, 'database> {
    pub meta: &'attribute ast::Meta,
    pub paths: &'attribute PathResolutions<'attribute, 'database>,
}

impl BuiltinAttribute<'_, '_> {
    pub fn is_named(&self, names: &[&str]) -> bool {
        self.meta
            .syntax()
            .children()
            .find_map(ast::Path::cast)
            .is_some_and(|path| {
                matches!(
                    self.paths.resolve(&path),
                    Some(PathResolution::BuiltinAttr(attribute))
                        if names
                            .iter()
                            .any(|name| attribute.name().symbol() == &Symbol::intern(name))
                )
            })
    }
}
