use ra_ap_hir::Crate;
use ra_ap_hir::Module;
use ra_ap_hir::ModuleDef;
use ra_ap_hir::ScopeDef;
use ra_ap_ide_db::RootDatabase;

fn child(database: &RootDatabase, module: Module, name: &str) -> Option<Module> {
    module.children(database).find(|child| {
        child
            .name(database)
            .is_some_and(|child_name| child_name.as_str() == name)
    })
}

pub fn standard_item(database: &RootDatabase, krate: Crate, path: &[&str]) -> Option<ModuleDef> {
    path.split_last().and_then(|(name, modules)| {
        modules
            .iter()
            .try_fold(krate.root_module(database), |module, segment| {
                child(database, module, segment)
            })
            .and_then(|module| {
                module
                    .scope(database, None)
                    .into_iter()
                    .find_map(|(item_name, definition)| match definition {
                        ScopeDef::ModuleDef(item) if item_name.as_str() == *name => Some(item),
                        _ => None,
                    })
            })
    })
}
