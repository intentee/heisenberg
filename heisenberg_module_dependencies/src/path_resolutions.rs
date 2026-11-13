use std::rc::Rc;

use ra_ap_hir::PathResolution;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::ast;

use crate::resolution_cache::ResolutionCache;
use crate::substituted_resolution::SubstitutedResolution;

pub struct PathResolutions<'resolutions, 'database> {
    resolved: ResolutionCache<
        ast::Path,
        Option<SubstitutedResolution<'database, PathResolution<'database>>>,
    >,
    semantics: &'resolutions Semantics<'database, RootDatabase>,
}

impl<'resolutions, 'database> PathResolutions<'resolutions, 'database> {
    pub fn new(semantics: &'resolutions Semantics<'database, RootDatabase>) -> Self {
        Self {
            resolved: ResolutionCache::default(),
            semantics,
        }
    }

    pub fn resolve(&self, path: &ast::Path) -> Option<PathResolution<'database>> {
        self.resolve_substituted(path)
            .map(|SubstitutedResolution { resolution, .. }| resolution)
    }

    pub fn resolve_substituted(
        &self,
        path: &ast::Path,
    ) -> Option<SubstitutedResolution<'database, PathResolution<'database>>> {
        self.resolved.resolve(path, || {
            self.semantics
                .resolve_path_with_subst(path)
                .map(|(resolution, substitution)| SubstitutedResolution {
                    resolution,
                    substitution: substitution.map(Rc::new),
                })
        })
    }
}
