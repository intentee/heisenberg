use std::cell::RefCell;
use std::collections::HashMap;

use ra_ap_hir::Module;
use ra_ap_ide_db::RootDatabase;

use heisenberg_module_graph::module_path::ModulePath;

use crate::is_module_inside_block::is_module_inside_block;

pub struct ModulePaths<'database> {
    database: &'database RootDatabase,
    paths: RefCell<HashMap<Module, ModulePath>>,
}

impl<'database> ModulePaths<'database> {
    pub fn new(database: &'database RootDatabase) -> Self {
        Self {
            database,
            paths: RefCell::default(),
        }
    }

    pub fn path(&self, module: Module) -> ModulePath {
        self.paths
            .borrow_mut()
            .entry(module)
            .or_insert_with(|| {
                let mut segments: Vec<String> = module
                    .path_to_root(self.database)
                    .iter()
                    .filter(|ancestor| !is_module_inside_block(self.database, **ancestor))
                    .filter_map(|ancestor| ancestor.name(self.database))
                    .map(|name| name.as_str().to_owned())
                    .collect();

                segments.reverse();

                ModulePath { segments }
            })
            .clone()
    }
}
