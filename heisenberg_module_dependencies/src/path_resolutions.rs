use std::cell::RefCell;
use std::collections::HashMap;

use ra_ap_hir::PathResolution;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::ast;

pub struct PathResolutions<'resolutions, 'database> {
    resolved: RefCell<HashMap<ast::Path, Option<PathResolution<'database>>>>,
    semantics: &'resolutions Semantics<'database, RootDatabase>,
}

impl<'resolutions, 'database> PathResolutions<'resolutions, 'database> {
    pub fn new(semantics: &'resolutions Semantics<'database, RootDatabase>) -> Self {
        Self {
            resolved: RefCell::default(),
            semantics,
        }
    }

    pub fn resolve(&self, path: &ast::Path) -> Option<PathResolution<'database>> {
        let known = self.resolved.borrow().get(path).copied();

        known.unwrap_or_else(|| {
            let resolution = self.semantics.resolve_path(path);

            self.resolved.borrow_mut().insert(path.clone(), resolution);

            resolution
        })
    }
}
