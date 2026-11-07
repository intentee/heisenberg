use ra_ap_cfg::CfgOptions;
use ra_ap_hir::HirFileId;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::NodeOrToken;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::WalkEvent;
use ra_ap_syntax::ast;

use crate::configured_node::ConfiguredNode;
use crate::token_origin::TokenOrigin;

pub struct CoveredCode<'code, 'database> {
    pub options: &'code CfgOptions,
    pub semantics: &'code Semantics<'database, RootDatabase>,
    pub token_origin: &'code TokenOrigin<'code, 'database>,
}

impl CoveredCode<'_, '_> {
    pub fn holds_workspace_text(&self, covered: &SyntaxNode) -> bool {
        let file = self.semantics.hir_file_for(covered);
        let mut preorder = covered.preorder_with_tokens();

        while let Some(event) = preorder.next() {
            match event {
                WalkEvent::Enter(NodeOrToken::Node(node)) => {
                    if ast::Attr::can_cast(node.kind())
                        || (ConfiguredNode {
                            node: &node,
                            options: self.options,
                        })
                        .is_disabled()
                    {
                        preorder.skip_subtree();
                    } else if let Some(expansion) =
                        ast::MacroCall::cast(node).and_then(|call| self.semantics.to_def(&call))
                    {
                        preorder.skip_subtree();

                        if self.holds_workspace_text(
                            &self.semantics.parse_or_expand(HirFileId::from(expansion)),
                        ) {
                            return true;
                        }
                    }
                }
                WalkEvent::Enter(NodeOrToken::Token(token)) => {
                    if !token.kind().is_trivia()
                        && self
                            .token_origin
                            .is_workspace_range(file, token.text_range())
                    {
                        return true;
                    }
                }
                WalkEvent::Leave(_) => {}
            }
        }

        false
    }
}
