use std::collections::BTreeSet;

use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::ast;

use heisenberg_module_graph::crate_target::CrateTarget;
use heisenberg_module_graph::function_body::FunctionBody;
use heisenberg_source_locator::source_locator::SourceLocator;

use crate::function_fingerprint::FunctionFingerprint;
use crate::item_focus::ItemFocus;
use crate::main_function::MainFunction;
use crate::module_dependencies_error::ModuleDependenciesError;
use crate::path_resolutions::PathResolutions;
use crate::template_captures::TemplateCaptures;

pub struct FunctionBodies<'bodies, 'database> {
    pub crate_target: &'bodies CrateTarget,
    pub locator: &'bodies SourceLocator<'bodies>,
    pub paths: &'bodies PathResolutions<'bodies, 'database>,
    pub semantics: &'bodies Semantics<'database, RootDatabase>,
    pub template_captures: &'bodies TemplateCaptures<'bodies, 'database>,
}

impl FunctionBodies<'_, '_> {
    pub fn written_in(
        &self,
        root: &SyntaxNode,
    ) -> Result<BTreeSet<FunctionBody>, ModuleDependenciesError> {
        let database = self.semantics.db;

        root.descendants()
            .filter_map(ast::Fn::cast)
            .filter(|function| {
                function
                    .body()
                    .and_then(|body| body.stmt_list())
                    .is_some_and(|statements| {
                        statements.statements().next().is_some() || statements.tail_expr().is_some()
                    })
                    && self.semantics.to_def(function).is_some_and(|definition| {
                        !MainFunction {
                            crate_target: self.crate_target,
                            database,
                            function: definition,
                        }
                        .enters_its_target()
                            && definition.as_proc_macro(database).is_none()
                    })
            })
            .map(|function| {
                FunctionFingerprint {
                    function: &function,
                    locator: self.locator,
                    paths: self.paths,
                    semantics: self.semantics,
                    template_captures: self.template_captures,
                }
                .tokens()
                .and_then(|fingerprint| {
                    self.locator
                        .locate(
                            ItemFocus {
                                node: function.syntax(),
                                semantics: self.semantics,
                            }
                            .original_range(),
                        )
                        .map(|location| FunctionBody {
                            fingerprint,
                            location,
                        })
                })
            })
            .collect()
    }
}
