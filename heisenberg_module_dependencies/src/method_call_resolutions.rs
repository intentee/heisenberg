use std::rc::Rc;

use ra_ap_hir::Function;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::ast;

use crate::resolution_cache::ResolutionCache;
use crate::substituted_resolution::SubstitutedResolution;

pub struct MethodCallResolutions<'resolutions, 'database> {
    resolved:
        ResolutionCache<ast::MethodCallExpr, Option<SubstitutedResolution<'database, Function>>>,
    semantics: &'resolutions Semantics<'database, RootDatabase>,
}

impl<'resolutions, 'database> MethodCallResolutions<'resolutions, 'database> {
    pub fn new(semantics: &'resolutions Semantics<'database, RootDatabase>) -> Self {
        Self {
            resolved: ResolutionCache::default(),
            semantics,
        }
    }

    pub fn resolve(
        &self,
        call: &ast::MethodCallExpr,
    ) -> Option<SubstitutedResolution<'database, Function>> {
        self.resolved.resolve(call, || {
            self.semantics
                .resolve_method_call_fallback(call)
                .and_then(|(callee, substitution)| {
                    callee.left().map(|function| SubstitutedResolution {
                        resolution: function,
                        substitution: substitution.map(Rc::new),
                    })
                })
        })
    }
}
