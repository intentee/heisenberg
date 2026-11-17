use std::rc::Rc;

use ra_ap_hir::ModuleDef;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::defs::Definition;
use ra_ap_ide_db::search::UsageSearchResult;

use crate::resolution_cache::ResolutionCache;

pub struct DefinitionUsages<'usages, 'database> {
    pub semantics: &'usages Semantics<'database, RootDatabase>,
    searched: ResolutionCache<Definition<'database>, Rc<UsageSearchResult>>,
    searched_with_self: ResolutionCache<Definition<'database>, Rc<UsageSearchResult>>,
}

impl<'usages, 'database> DefinitionUsages<'usages, 'database> {
    pub fn new(semantics: &'usages Semantics<'database, RootDatabase>) -> Self {
        Self {
            semantics,
            searched: ResolutionCache::default(),
            searched_with_self: ResolutionCache::default(),
        }
    }

    pub fn all(&self, definition: ModuleDef) -> Rc<UsageSearchResult> {
        self.of(Definition::from(definition))
    }

    pub fn of(&self, definition: Definition<'database>) -> Rc<UsageSearchResult> {
        self.searched.resolve(&definition, || {
            Rc::new(definition.usages(self.semantics).all())
        })
    }

    pub fn with_self_references(&self, definition: Definition<'database>) -> Rc<UsageSearchResult> {
        self.searched_with_self.resolve(&definition, || {
            Rc::new(definition.usages(self.semantics).include_self_refs().all())
        })
    }
}
