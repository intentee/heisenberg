use ra_ap_hir::PathResolution;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::ast;

use crate::path_resolutions::PathResolutions;

pub struct ToolAttribute<'attribute, 'database> {
    pub meta: &'attribute ast::Meta,
    pub paths: &'attribute PathResolutions<'attribute, 'database>,
    pub semantics: &'attribute Semantics<'database, RootDatabase>,
}

impl ToolAttribute<'_, '_> {
    pub fn tool(&self, tools: &[&str]) -> Option<String> {
        self.meta
            .syntax()
            .children()
            .find_map(ast::Path::cast)
            .and_then(|path| match self.paths.resolve(&path) {
                Some(PathResolution::ToolModule(tool)) => {
                    Some(tool.name(self.semantics.db).as_str().to_owned())
                }
                _ => None,
            })
            .filter(|name| tools.contains(&name.as_str()))
    }
}
