use std::sync::Arc;

use ra_ap_cfg::CfgOptions;
use ra_ap_hir::Crate;
use ra_ap_hir::InFile;
use ra_ap_syntax::SyntaxNodePtr;

use heisenberg_task_graph::once_map::OnceMap;

#[derive(Default)]
pub struct TestModules {
    pub enclosing: OnceMap<InFile<SyntaxNodePtr>, bool>,
    pub production_options: OnceMap<Crate, Arc<CfgOptions>>,
}
