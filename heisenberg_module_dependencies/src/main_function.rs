use ra_ap_hir::Function;
use ra_ap_ide_db::RootDatabase;

use heisenberg_module_graph::crate_target::CrateTarget;

pub struct MainFunction<'main> {
    pub crate_target: &'main CrateTarget,
    pub database: &'main RootDatabase,
    pub function: Function,
}

impl MainFunction<'_> {
    pub fn enters_its_target(&self) -> bool {
        self.crate_target.enters_through_main() && self.function.is_main(self.database)
    }
}
