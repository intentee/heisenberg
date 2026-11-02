use ra_ap_hir::Module;
use ra_ap_hir::ModuleId;
use ra_ap_ide_db::RootDatabase;

pub fn is_module_inside_block(database: &RootDatabase, module: Module) -> bool {
    ModuleId::from(module).block(database).is_some()
}
