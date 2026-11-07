use std::cell::OnceCell;

use ra_ap_hir::ModuleDef;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::defs::Definition;
use ra_ap_ide_db::search::UsageSearchResult;

pub struct DefinitionUsages<'usages, 'database> {
    pub definition: ModuleDef,
    pub semantics: &'usages Semantics<'database, RootDatabase>,
    usages: OnceCell<UsageSearchResult>,
}

impl<'usages, 'database> DefinitionUsages<'usages, 'database> {
    pub const fn new(
        definition: ModuleDef,
        semantics: &'usages Semantics<'database, RootDatabase>,
    ) -> Self {
        Self {
            definition,
            semantics,
            usages: OnceCell::new(),
        }
    }

    pub fn all(&self) -> &UsageSearchResult {
        self.usages.get_or_init(|| {
            Definition::from(self.definition)
                .usages(self.semantics)
                .all()
        })
    }
}
