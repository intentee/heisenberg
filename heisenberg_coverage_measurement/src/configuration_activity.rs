use ra_ap_cfg::CfgOptions;
use ra_ap_hir::HirFileId;
use ra_ap_hir::InFile;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::SyntaxNodePtr;

use heisenberg_configured_code::configured_node::ConfiguredNode;
use heisenberg_task_graph::once_map::OnceMap;

pub struct ConfigurationActivity<'activity> {
    pub active: &'activity OnceMap<InFile<SyntaxNodePtr>, bool>,
    pub file_id: HirFileId,
    pub options: &'activity CfgOptions,
}

impl ConfigurationActivity<'_> {
    pub fn is_active(&self, node: &SyntaxNode) -> bool {
        self.active
            .get_or_init(InFile::new(self.file_id, SyntaxNodePtr::new(node)), || {
                !ConfiguredNode {
                    node,
                    options: self.options,
                }
                .is_disabled()
                    && node.parent().is_none_or(|parent| self.is_active(&parent))
            })
    }
}
