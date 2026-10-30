use ra_ap_hir::Module;
use ra_ap_ide_db::RootDatabase;
use ra_ap_intern::Symbol;

use heisenberg_module_graph::constants::CONSTANTS_MODULE_NAME;

pub struct ConstantsModule<'database> {
    pub database: &'database RootDatabase,
}

impl ConstantsModule<'_> {
    pub fn matches(&self, module: Module) -> bool {
        module.parent(self.database) == Some(module.krate(self.database).root_module(self.database))
            && module
                .name(self.database)
                .is_some_and(|name| name.symbol() == &Symbol::intern(CONSTANTS_MODULE_NAME))
    }
}
