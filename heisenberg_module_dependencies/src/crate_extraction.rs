use std::collections::BTreeSet;

use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_module_graph::crate_target::CrateTarget;
use heisenberg_module_graph::module_dependency::ModuleDependency;
use heisenberg_module_graph::source_location::SourceLocation;
use heisenberg_module_graph::violation::Violation;
use heisenberg_module_graph::violation_kind::ViolationKind;
use ra_ap_hir::Crate;
use ra_ap_hir::FileRangeWrapper;
use ra_ap_hir::MacroKind;
use ra_ap_hir::Module;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::TextRange;
use ra_ap_syntax::TextSize;
use ra_ap_syntax::WalkEvent;
use ra_ap_syntax::ast;
use ra_ap_vfs::FileId;

use crate::configured_node::ConfiguredNode;
use crate::diagnostic_violation::DiagnosticViolation;
use crate::module_paths::ModulePaths;
use crate::path_reference::PathReference;
use crate::path_reference_outcome::PathReferenceOutcome;
use crate::source_locator::SourceLocator;
use crate::subtree::Subtree;
use crate::subtree_origin::SubtreeOrigin;
use crate::token_origin::TokenOrigin;

pub struct CrateExtraction<'extraction, 'database> {
    pub crate_target: &'extraction CrateTarget,
    pub krate: Crate,
    pub locator: &'extraction SourceLocator<'extraction>,
    pub semantics: &'extraction Semantics<'database, RootDatabase>,
}

impl<'database> CrateExtraction<'_, 'database> {
    pub fn graph(&self) -> CrateModuleGraph {
        let database = self.semantics.db;
        let module_paths = ModulePaths { database };
        let modules = self.krate.modules(database);
        let own_files: BTreeSet<FileId> = modules
            .iter()
            .filter_map(|module| module.as_source_file_id(database))
            .map(|source_file_id| source_file_id.file_id(database))
            .collect();
        let token_origin = TokenOrigin {
            own_files: &own_files,
            semantics: self.semantics,
        };
        let mut dependencies = BTreeSet::new();
        let mut violations = BTreeSet::new();

        for module in &modules {
            self.module_source(
                *module,
                &module_paths,
                &token_origin,
                &mut dependencies,
                &mut violations,
            );

            let mut diagnostics = Vec::new();

            module.diagnostics(database, &mut diagnostics, false);
            violations.extend(diagnostics.iter().filter_map(|diagnostic| {
                DiagnosticViolation {
                    diagnostic,
                    locator: self.locator,
                    semantics: self.semantics,
                    token_origin: &token_origin,
                }
                .violation()
            }));
        }

        CrateModuleGraph {
            crate_target: self.crate_target.clone(),
            dependencies,
            modules: modules
                .iter()
                .map(|module| module_paths.path(*module))
                .collect(),
            violations,
        }
    }

    fn module_source(
        &self,
        module: Module,
        module_paths: &ModulePaths<'_>,
        token_origin: &TokenOrigin<'_, 'database>,
        dependencies: &mut BTreeSet<ModuleDependency>,
        violations: &mut BTreeSet<Violation>,
    ) {
        let database = self.semantics.db;

        if let Some(source_file_id) = module.as_source_file_id(database) {
            let file_id = source_file_id.file_id(database);

            if self.semantics.file_to_module_defs(file_id).nth(1).is_some() {
                violations.insert(Violation {
                    kind: ViolationKind::SharedModuleFile,
                    location: self.locator.locate(FileRangeWrapper {
                        file_id,
                        range: TextRange::empty(TextSize::from(0)),
                    }),
                });
            } else {
                self.walk(
                    Subtree {
                        origin: SubtreeOrigin::SourceFile,
                        root: self.semantics.parse(source_file_id).syntax().clone(),
                    },
                    module_paths,
                    token_origin,
                    dependencies,
                    violations,
                );
            }
        }
    }

    fn walk(
        &self,
        root: Subtree,
        module_paths: &ModulePaths<'_>,
        token_origin: &TokenOrigin<'_, 'database>,
        dependencies: &mut BTreeSet<ModuleDependency>,
        violations: &mut BTreeSet<Violation>,
    ) {
        let options = self.krate.cfg(self.semantics.db);
        let mut pending = vec![root];

        while let Some(subtree) = pending.pop() {
            let mut preorder = subtree.root.preorder();

            while let Some(event) = preorder.next() {
                if let WalkEvent::Enter(node) = event {
                    let configured = ConfiguredNode {
                        node: &node,
                        options,
                    };

                    violations.extend(
                        configured
                            .negated_attributes()
                            .iter()
                            .filter(|attribute| !token_origin.is_foreign_text(attribute.syntax()))
                            .map(|attribute| Violation {
                                kind: ViolationKind::NegatedConfiguration,
                                location: self.locate(
                                    attribute.syntax(),
                                    &subtree.origin,
                                    token_origin,
                                ),
                            }),
                    );

                    if configured.is_disabled() {
                        preorder.skip_subtree();
                    } else if let Some(path) = ast::Path::cast(node.clone()) {
                        self.record(
                            &path,
                            &subtree.origin,
                            module_paths,
                            token_origin,
                            dependencies,
                            violations,
                        );
                    } else if let Some(call) = ast::MacroCall::cast(node.clone()) {
                        self.expand(
                            &call,
                            &subtree.origin,
                            &mut pending,
                            token_origin,
                            violations,
                        );
                    } else if let Some(tree) = ast::UseTree::cast(node).filter(|tree| {
                        tree.star_token().is_some_and(|star| {
                            token_origin.is_own_text(
                                self.semantics.hir_file_for(tree.syntax()),
                                star.text_range(),
                            )
                        })
                    }) {
                        violations.insert(Violation {
                            kind: ViolationKind::GlobImport,
                            location: self.locate(tree.syntax(), &subtree.origin, token_origin),
                        });
                    }
                }
            }
        }
    }

    fn expand(
        &self,
        call: &ast::MacroCall,
        origin: &SubtreeOrigin,
        pending: &mut Vec<Subtree>,
        token_origin: &TokenOrigin<'_, 'database>,
        violations: &mut BTreeSet<Violation>,
    ) {
        let database = self.semantics.db;
        let is_procedural = self
            .semantics
            .resolve_macro_call(call)
            .is_some_and(|definition| matches!(definition.kind(database), MacroKind::ProcMacro));

        if let Some(expansion) = self.semantics.to_def(call).filter(|_| !is_procedural) {
            let (parse, _) = &expansion.parse_macro_expansion(database).value;

            if !parse.errors().is_empty() && !token_origin.is_foreign_text(call.syntax()) {
                violations.insert(Violation {
                    kind: ViolationKind::UnparsableMacroExpansion,
                    location: self.locate(call.syntax(), origin, token_origin),
                });
            }

            pending.push(Subtree {
                origin: match origin {
                    SubtreeOrigin::MacroCall(outermost) => {
                        SubtreeOrigin::MacroCall(outermost.clone())
                    }
                    SubtreeOrigin::SourceFile => SubtreeOrigin::MacroCall(call.clone()),
                },
                root: self.semantics.expand(expansion).value,
            });
        }
    }

    fn record(
        &self,
        path: &ast::Path,
        origin: &SubtreeOrigin,
        module_paths: &ModulePaths<'_>,
        token_origin: &TokenOrigin<'_, 'database>,
        dependencies: &mut BTreeSet<ModuleDependency>,
        violations: &mut BTreeSet<Violation>,
    ) {
        match (PathReference {
            krate: self.krate,
            path,
            scope: match origin {
                SubtreeOrigin::MacroCall(outermost) => outermost.syntax(),
                SubtreeOrigin::SourceFile => path.syntax(),
            },
            semantics: self.semantics,
            token_origin,
        })
        .outcome()
        {
            PathReferenceOutcome::Dependency(referenced) => {
                dependencies.insert(ModuleDependency {
                    from: module_paths.path(referenced.from),
                    location: self.locate(path.syntax(), origin, token_origin),
                    to: module_paths.path(referenced.to),
                });
            }
            PathReferenceOutcome::Irrelevant => {}
            PathReferenceOutcome::Unresolved => {
                violations.insert(Violation {
                    kind: ViolationKind::UnresolvedPath,
                    location: self.locate(path.syntax(), origin, token_origin),
                });
            }
        }
    }

    fn locate(
        &self,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
        token_origin: &TokenOrigin<'_, 'database>,
    ) -> SourceLocation {
        let database = self.semantics.db;
        let original = self.semantics.original_range(node);
        let file_id = original.file_id.file_id(database);

        match origin {
            SubtreeOrigin::MacroCall(outermost) if !token_origin.is_own_file(file_id) => {
                let call = self.semantics.original_range(outermost.syntax());

                self.locator.locate(FileRangeWrapper {
                    file_id: call.file_id.file_id(database),
                    range: call.range,
                })
            }
            SubtreeOrigin::MacroCall(_) | SubtreeOrigin::SourceFile => {
                self.locator.locate(FileRangeWrapper {
                    file_id,
                    range: original.range,
                })
            }
        }
    }
}
