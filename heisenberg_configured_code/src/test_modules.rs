use std::sync::Arc;

use ra_ap_cfg::CfgOptions;
use ra_ap_hir::Crate;
use ra_ap_hir::InFile;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::SyntaxNodePtr;

use heisenberg_task_graph::once_map::OnceMap;

use crate::production_options::production_options;

#[derive(Default)]
pub struct TestModules {
    pub(crate) enclosing: OnceMap<InFile<SyntaxNodePtr>, bool>,
    production_options: OnceMap<Crate, Arc<CfgOptions>>,
}

impl TestModules {
    pub fn production_options(&self, database: &RootDatabase, krate: Crate) -> Arc<CfgOptions> {
        self.production_options
            .get_or_init(krate, || Arc::new(production_options(krate.cfg(database))))
    }
}
