use std::collections::BTreeSet;
use std::collections::HashMap;

use ra_ap_cfg::CfgExpr;
use ra_ap_hir::AnyDiagnostic;
use ra_ap_hir::Crate;
use ra_ap_hir::FileRangeWrapper;
use ra_ap_hir::HirFileId;
use ra_ap_hir::InFile;
use ra_ap_hir::Module;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::base_db::EditionedFileId;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxNodePtr;
use ra_ap_syntax::ast;
use ra_ap_vfs::FileId;

use heisenberg_module_graph::configured_out_negations::ConfiguredOutNegations;
use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_module_graph::crate_target::CrateTarget;
use heisenberg_module_graph::folded_finding::FoldedFinding;
use heisenberg_module_graph::folded_finding_kind::FoldedFindingKind;
use heisenberg_module_graph::function_body::FunctionBody;
use heisenberg_module_graph::source_location::SourceLocation;
use heisenberg_module_graph::violation::Violation;
use heisenberg_module_graph::violation_kind::ViolationKind;
use heisenberg_module_graph::violation_set::ViolationSet;
use heisenberg_module_graph::walked_code::WalkedCode;

use crate::covering_ancestors::CoveringAncestors;
use crate::crate_module_files::crate_module_files;
use crate::diagnostic_violation::DiagnosticViolation;
use crate::function_fingerprint::FunctionFingerprint;
use crate::item_focus::ItemFocus;
use crate::located_violation::LocatedViolation;
use crate::module_paths::ModulePaths;
use crate::module_walk::ModuleWalk;
use crate::production_options::production_options;
use crate::source_locator::SourceLocator;
use crate::test_code::TestCode;
use crate::test_target_findings::TestTargetFindings;
use crate::token_origin::TokenOrigin;

pub struct CrateExtraction<'extraction, 'database> {
    pub cargo_variables: &'extraction BTreeSet<String>,
    pub crate_target: &'extraction CrateTarget,
    pub krate: Crate,
    pub locator: &'extraction SourceLocator<'extraction>,
    pub semantics: &'extraction Semantics<'database, RootDatabase>,
    pub test_code_files: &'extraction BTreeSet<FileId>,
    pub unpublished_packages: &'extraction BTreeSet<String>,
    pub workspace_files: &'extraction BTreeSet<FileId>,
}

impl CrateExtraction<'_, '_> {
    pub fn graph(&self) -> CrateModuleGraph {
        let database = self.semantics.db;
        let module_paths = ModulePaths { database };
        let modules = self.krate.modules(database);
        let own_files = crate_module_files(database, self.krate);
        let token_origin = TokenOrigin {
            own_files: &own_files,
            semantics: self.semantics,
            workspace_files: self.workspace_files,
        };
        let options = self.krate.cfg(database);
        let test_code = TestCode {
            files: self.test_code_files,
            semantics: self.semantics,
        };
        let mut diagnostics = Vec::new();

        for module in &modules {
            module.diagnostics(database, &mut diagnostics, false);
        }

        let mut inactive_code: HashMap<InFile<SyntaxNodePtr>, CfgExpr> = HashMap::new();

        for diagnostic in &diagnostics {
            if let AnyDiagnostic::InactiveCode(inactive) = diagnostic {
                inactive_code.insert(inactive.node, inactive.cfg.clone());
            }
        }

        let walk = ModuleWalk {
            cargo_variables: self.cargo_variables,
            crate_target: self.crate_target,
            inactive_code: &inactive_code,
            krate: self.krate,
            locator: self.locator,
            module_paths: &module_paths,
            options,
            production_options: &production_options(options),
            semantics: self.semantics,
            test_code: &test_code,
            token_origin: &token_origin,
            unpublished_packages: self.unpublished_packages,
        };
        let root = self.krate.root_module(database);
        let mut graph = CrateModuleGraph {
            configured_out_negations: ConfiguredOutNegations::default(),
            crate_target: self.crate_target.clone(),
            dependencies: BTreeSet::new(),
            function_bodies: self.function_bodies(&modules),
            modules: modules
                .iter()
                .map(|module| module_paths.path(*module))
                .collect(),
            root_module_is_empty: root.declarations(database).is_empty()
                && root.impl_defs(database).is_empty()
                && root.legacy_macros(database).is_empty()
                && root.scope(database, None).is_empty(),
            shared_module_files: BTreeSet::new(),
            unresolved_foreign_references: BTreeSet::new(),
            violations: ViolationSet::default(),
            walked_code: WalkedCode::default(),
        };
        let mut test_target = TestTargetFindings::default();

        for module in &modules {
            walk.module(*module, &mut graph, &mut test_target);
        }

        test_target.fold(&mut graph);

        let located: Vec<LocatedViolation> = diagnostics
            .iter()
            .filter_map(|diagnostic| {
                DiagnosticViolation {
                    diagnostic,
                    semantics: self.semantics,
                    token_origin: &token_origin,
                }
                .violation()
            })
            .collect();

        for LocatedViolation { display, kind } in &located {
            let location = self.locator.locate(*display);

            if *kind == ViolationKind::FailedMacroExpansion {
                let violation = Violation {
                    kind: ViolationKind::FailedMacroExpansion,
                    location: self.written_call(*display),
                };

                if violation.location == location {
                    graph.violations.insert(violation);
                } else {
                    graph.violations.fold(
                        violation,
                        FoldedFinding {
                            kind: FoldedFindingKind::FailingExpansion,
                            location,
                        },
                    );
                }
            } else {
                graph.violations.insert(Violation {
                    kind: kind.clone(),
                    location,
                });
            }
        }

        graph
    }

    fn function_bodies(&self, modules: &[Module]) -> BTreeSet<FunctionBody> {
        let database = self.semantics.db;

        modules
            .iter()
            .filter_map(|module| module.as_source_file_id(database))
            .flat_map(|source_file_id| {
                self.semantics
                    .parse(source_file_id)
                    .syntax()
                    .descendants()
                    .filter_map(ast::Fn::cast)
                    .collect::<Vec<ast::Fn>>()
            })
            .filter(|function| {
                function
                    .body()
                    .and_then(|body| body.stmt_list())
                    .is_some_and(|statements| {
                        statements.statements().next().is_some() || statements.tail_expr().is_some()
                    })
                    && self.semantics.to_def(function).is_some()
            })
            .map(|function| FunctionBody {
                fingerprint: FunctionFingerprint {
                    function: &function,
                    krate: self.krate,
                    locator: self.locator,
                    semantics: self.semantics,
                }
                .tokens(),
                location: self.locator.locate(
                    ItemFocus {
                        node: function.syntax(),
                        semantics: self.semantics,
                    }
                    .original_range(),
                ),
            })
            .collect()
    }

    fn written_call(&self, display: FileRangeWrapper<FileId>) -> SourceLocation {
        let database = self.semantics.db;

        CoveringAncestors {
            range: InFile::new(
                HirFileId::from(EditionedFileId::new(
                    database,
                    display.file_id,
                    self.krate.edition(database),
                )),
                display.range,
            ),
            semantics: self.semantics,
        }
        .nodes()
        .filter_map(ast::MacroCall::cast)
        .last()
        .map_or_else(
            || self.locator.locate(display),
            |call| {
                self.locator.locate(
                    ItemFocus {
                        node: call.syntax(),
                        semantics: self.semantics,
                    }
                    .original_range(),
                )
            },
        )
    }
}
