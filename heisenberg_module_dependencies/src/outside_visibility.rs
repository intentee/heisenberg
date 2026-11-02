use ra_ap_hir::Module;
use ra_ap_hir::ModuleId;
use ra_ap_hir::Visibility;
use ra_ap_ide_db::RootDatabase;

pub struct OutsideVisibility<'database> {
    pub database: &'database RootDatabase,
}

impl OutsideVisibility<'_> {
    pub fn reaches_outside(&self, visibility: Visibility, module: Module) -> bool {
        if module.is_crate_root(self.database) {
            visibility.is_public()
        } else {
            !matches!(visibility, Visibility::Module(owner, _) if owner == ModuleId::from(module))
        }
    }
}
