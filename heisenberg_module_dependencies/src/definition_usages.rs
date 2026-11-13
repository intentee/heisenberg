use std::rc::Rc;

use ra_ap_hir::ModuleDef;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::defs::Definition;
use ra_ap_ide_db::search::UsageSearchResult;

use crate::resolution_cache::ResolutionCache;

pub struct DefinitionUsages<'usages, 'database> {
    pub semantics: &'usages Semantics<'database, RootDatabase>,
    searched: ResolutionCache<ModuleDef, Rc<UsageSearchResult>>,
}

impl<'usages, 'database> DefinitionUsages<'usages, 'database> {
    pub fn new(semantics: &'usages Semantics<'database, RootDatabase>) -> Self {
        Self {
            semantics,
            searched: ResolutionCache::default(),
        }
    }

    pub fn all(&self, definition: ModuleDef) -> Rc<UsageSearchResult> {
        self.searched.resolve(&definition, || {
            Rc::new(Definition::from(definition).usages(self.semantics).all())
        })
    }
}
