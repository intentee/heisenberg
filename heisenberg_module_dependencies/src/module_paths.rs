use ra_ap_hir::Module;
use ra_ap_ide_db::RootDatabase;

use heisenberg_module_graph::module_path::ModulePath;
use heisenberg_task_graph::once_map::OnceMap;

use crate::is_module_inside_block::is_module_inside_block;

#[derive(Default)]
pub struct ModulePaths {
    paths: OnceMap<Module, ModulePath>,
}

impl ModulePaths {
    pub fn path(&self, database: &RootDatabase, module: Module) -> ModulePath {
        self.paths.get_or_init(module, || {
            let mut segments: Vec<String> = module
                .path_to_root(database)
                .iter()
                .filter(|ancestor| !is_module_inside_block(database, **ancestor))
                .filter_map(|ancestor| ancestor.name(database))
                .map(|name| name.as_str().to_owned())
                .collect();

            segments.reverse();

            ModulePath { segments }
        })
    }
}
