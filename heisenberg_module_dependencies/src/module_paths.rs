use ra_ap_hir::Module;
use ra_ap_ide_db::RootDatabase;

use heisenberg_module_graph::module_path::ModulePath;

use crate::is_module_inside_block::is_module_inside_block;

pub struct ModulePaths<'database> {
    pub database: &'database RootDatabase,
}

impl ModulePaths<'_> {
    pub fn path(&self, module: Module) -> ModulePath {
        let mut segments: Vec<String> = module
            .path_to_root(self.database)
            .iter()
            .filter(|ancestor| !is_module_inside_block(self.database, **ancestor))
            .filter_map(|ancestor| ancestor.name(self.database))
            .map(|name| name.as_str().to_owned())
            .collect();

        segments.reverse();

        ModulePath { segments }
    }
}
