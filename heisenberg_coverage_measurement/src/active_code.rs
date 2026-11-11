use ra_ap_hir::InFile;
use ra_ap_syntax::SyntaxNodePtr;

use heisenberg_task_graph::once_map::OnceMap;

#[derive(Default)]
pub struct ActiveCode {
    pub crate_build: OnceMap<InFile<SyntaxNodePtr>, bool>,
    pub production_build: OnceMap<InFile<SyntaxNodePtr>, bool>,
}
