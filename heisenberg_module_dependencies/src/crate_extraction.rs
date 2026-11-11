use std::collections::BTreeSet;

use ra_ap_hir::Crate;
use ra_ap_hir::FileRangeWrapper;
use ra_ap_hir::HirFileId;
use ra_ap_hir::InFile;
use ra_ap_hir::Module;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::base_db::EditionedFileId;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::ast;
use ra_ap_vfs::FileId;

use heisenberg_expanded_code::covering_ancestors::CoveringAncestors;
use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_module_graph::crate_target::CrateTarget;
use heisenberg_module_graph::folded_finding::FoldedFinding;
use heisenberg_module_graph::folded_finding_kind::FoldedFindingKind;
use heisenberg_module_graph::module_findings::ModuleFindings;
use heisenberg_module_graph::source_location::SourceLocation;
use heisenberg_module_graph::violation::Violation;
use heisenberg_module_graph::violation_kind::ViolationKind;
use heisenberg_source_locator::source_locator::SourceLocator;

use crate::crate_findings::CrateFindings;
use crate::diagnostic_site::DiagnosticSite;
use crate::item_focus::ItemFocus;
use crate::located_violation::LocatedViolation;
use crate::module_dependencies_error::ModuleDependenciesError;
use crate::module_diagnostics::ModuleDiagnostics;
use crate::module_paths::ModulePaths;
use crate::token_origin::TokenOrigin;
use crate::walked_module::WalkedModule;

pub struct CrateExtraction<'extraction, 'database> {
    pub crate_target: &'extraction CrateTarget,
    pub diagnostics: &'extraction [ModuleDiagnostics],
    pub files: &'extraction BTreeSet<FileId>,
    pub krate: Crate,
    pub locator: &'extraction SourceLocator<'extraction>,
    pub module_paths: &'extraction ModulePaths,
    pub modules: &'extraction [Module],
    pub semantics: &'extraction Semantics<'database, RootDatabase>,
    pub workspace_files: &'extraction BTreeSet<FileId>,
}

impl CrateExtraction<'_, '_> {
    pub fn graph(
        &self,
        walked: Vec<Result<WalkedModule, ModuleDependenciesError>>,
    ) -> Result<CrateModuleGraph, ModuleDependenciesError> {
        let database = self.semantics.db;
        let token_origin = TokenOrigin {
            own_files: self.files,
            semantics: self.semantics,
            workspace_files: self.workspace_files,
        };
        let root = self.krate.root_module(database);

        let mut crate_findings = CrateFindings::default();
        let mut findings = ModuleFindings::default();
        let mut function_bodies = BTreeSet::new();

        walked
            .into_iter()
            .try_for_each(|module| {
                module.map(
                    |WalkedModule {
                         crate_findings: module_crate_findings,
                         findings: module_findings,
                         function_bodies: module_function_bodies,
                     }| {
                        crate_findings.merge(module_crate_findings);
                        findings.merge(module_findings);
                        function_bodies.extend(module_function_bodies);
                    },
                )
            })
            .and_then(|()| {
                crate_findings.fold(&mut findings);

                self.diagnostics
                    .iter()
                    .flat_map(|ModuleDiagnostics { sites, .. }| sites)
                    .filter(|DiagnosticSite { origin, .. }| {
                        token_origin.is_own_text(origin.file_id, origin.value)
                    })
                    .try_for_each(
                        |DiagnosticSite {
                             violation: LocatedViolation { display, kind },
                             ..
                         }| {
                            self.locator.locate(*display).and_then(|location| {
                                if *kind == ViolationKind::FailedMacroExpansion {
                                    self.written_call(*display).map(|written| {
                                        let violation = Violation {
                                            kind: ViolationKind::FailedMacroExpansion,
                                            location: written,
                                        };

                                        if violation.location == location {
                                            findings.violations.insert(violation);
                                        } else {
                                            findings.violations.fold(
                                                violation,
                                                FoldedFinding {
                                                    kind: FoldedFindingKind::FailingExpansion,
                                                    location,
                                                },
                                            );
                                        }
                                    })
                                } else {
                                    findings.violations.insert(Violation {
                                        kind: kind.clone(),
                                        location,
                                    });

                                    Ok(())
                                }
                            })
                        },
                    )
            })
            .map(|()| CrateModuleGraph {
                crate_target: self.crate_target.clone(),
                findings,
                function_bodies,
                modules: self
                    .modules
                    .iter()
                    .map(|module| self.module_paths.path(database, *module))
                    .collect(),
                root_module_is_empty: root.declarations(database).is_empty()
                    && root.impl_defs(database).is_empty()
                    && root.legacy_macros(database).is_empty()
                    && root.scope(database, None).is_empty(),
            })
    }

    fn written_call(
        &self,
        display: FileRangeWrapper<FileId>,
    ) -> Result<SourceLocation, ModuleDependenciesError> {
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
