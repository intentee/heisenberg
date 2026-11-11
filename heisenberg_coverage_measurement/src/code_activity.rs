use std::ops::ControlFlow;
use std::sync::Arc;

use ra_ap_cfg::CfgOptions;
use ra_ap_hir::Crate;
use ra_ap_hir::HirFileId;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::ast;

use heisenberg_configured_code::visit_active_metas::visit_active_metas;

use crate::active_code::ActiveCode;
use crate::analysis_context::AnalysisContext;
use crate::configuration_activity::ConfigurationActivity;

pub struct CodeActivity<'activity> {
    active_code: &'activity ActiveCode,
    crate_options: &'activity CfgOptions,
    file_id: HirFileId,
    production_options: Arc<CfgOptions>,
}

impl<'activity> CodeActivity<'activity> {
    pub fn new(
        AnalysisContext {
            active_code,
            test_modules,
            ..
        }: &AnalysisContext<'activity, '_>,
        file_id: HirFileId,
        krate: Crate,
    ) -> Self {
        let database = test_modules.semantics.db;

        Self {
            active_code,
            crate_options: krate.cfg(database),
            file_id,
            production_options: test_modules.modules.production_options(database, krate),
        }
    }

    pub fn first_production_only_meta(&self, node: &SyntaxNode) -> Option<ast::Meta> {
        let mut analysed = Vec::new();

        visit_active_metas(node, self.crate_options, |meta| {
            analysed.push(meta);

            ControlFlow::<()>::Continue(())
        });

        visit_active_metas(node, &self.production_options, |meta| {
            if analysed.contains(&meta) {
                ControlFlow::Continue(())
            } else {
                ControlFlow::Break(meta)
            }
        })
    }

    pub fn is_active(&self, node: &SyntaxNode) -> bool {
        self.crate_build().is_active(node) || self.production_build().is_active(node)
    }

    pub fn is_production_only(&self, node: &SyntaxNode) -> bool {
        !self.crate_build().is_active(node) && self.production_build().is_active(node)
    }

    const fn crate_build(&self) -> ConfigurationActivity<'_> {
        ConfigurationActivity {
            active: &self.active_code.crate_build,
            file_id: self.file_id,
            options: self.crate_options,
        }
    }

    fn production_build(&self) -> ConfigurationActivity<'_> {
        ConfigurationActivity {
            active: &self.active_code.production_build,
            file_id: self.file_id,
            options: &self.production_options,
        }
    }
}
