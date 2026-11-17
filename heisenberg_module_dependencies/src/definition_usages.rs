use std::rc::Rc;

use ra_ap_hir::ModuleDef;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::defs::Definition;

use crate::found_references::FoundReferences;
use crate::reference_search::ReferenceSearch;
use crate::resolution_cache::ResolutionCache;

pub struct DefinitionUsages<'usages, 'database> {
    pub search: ReferenceSearch<'usages, 'database>,
    pub semantics: &'usages Semantics<'database, RootDatabase>,
    searched: ResolutionCache<Definition<'database>, Rc<FoundReferences>>,
    searched_with_self: ResolutionCache<Definition<'database>, Rc<FoundReferences>>,
}

impl<'usages, 'database> DefinitionUsages<'usages, 'database> {
    pub fn new(search: ReferenceSearch<'usages, 'database>) -> Self {
        Self {
            semantics: search.semantics,
            search,
            searched: ResolutionCache::default(),
            searched_with_self: ResolutionCache::default(),
        }
    }

    pub fn all(&self, definition: ModuleDef) -> Rc<FoundReferences> {
        self.of(Definition::from(definition))
    }

    pub fn of(&self, definition: Definition<'database>) -> Rc<FoundReferences> {
        self.searched.resolve(&definition, || {
            Rc::new(self.search.references(definition, false))
        })
    }

    pub fn with_self_references(&self, definition: Definition<'database>) -> Rc<FoundReferences> {
        self.searched_with_self.resolve(&definition, || {
            Rc::new(self.search.references(definition, true))
        })
    }
}
