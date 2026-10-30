use ra_ap_cfg::CfgOptions;
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
use ra_ap_syntax::ast::HasVisibility;
use ra_ap_vfs::FileId;

use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_module_graph::crate_target_kind::CrateTargetKind;
use heisenberg_module_graph::inactive_region::InactiveRegion;
use heisenberg_module_graph::module_dependency::ModuleDependency;
use heisenberg_module_graph::module_file_analysis::ModuleFileAnalysis;
use heisenberg_module_graph::shared_module_file::SharedModuleFile;
use heisenberg_module_graph::source_location::SourceLocation;
use heisenberg_module_graph::violation::Violation;
use heisenberg_module_graph::violation_kind::ViolationKind;

use crate::configured_node::ConfiguredNode;
use crate::constants_module::ConstantsModule;
use crate::implemented_type::ImplementedType;
use crate::module_paths::ModulePaths;
use crate::path_reference::PathReference;
use crate::path_reference_outcome::PathReferenceOutcome;
use crate::public_items::PublicItems;
use crate::source_locator::SourceLocator;
use crate::subtree::Subtree;
use crate::subtree_origin::SubtreeOrigin;
use crate::test_target_items::TestTargetItems;
use crate::token_origin::TokenOrigin;

pub struct ModuleWalk<'walk, 'database> {
    pub crate_target_kind: CrateTargetKind,
    pub krate: Crate,
    pub locator: &'walk SourceLocator<'walk>,
    pub module_paths: &'walk ModulePaths<'database>,
    pub options: &'walk CfgOptions,
    pub semantics: &'walk Semantics<'database, RootDatabase>,
    pub token_origin: &'walk TokenOrigin<'walk, 'database>,
}

impl ModuleWalk<'_, '_> {
    #[expect(
        clippy::expect_used,
        reason = "rust-analyzer maps the file of a module back to that module"
    )]
    pub fn module(&self, module: Module, graph: &mut CrateModuleGraph) {
        let database = self.semantics.db;

        if let Some(source_file_id) = module.as_source_file_id(database) {
            let file_id = source_file_id.file_id(database);
            let file_modules: Vec<Module> = self.semantics.file_to_module_defs(file_id).collect();
            let file = self.file_start(file_id).file;
            let first = *file_modules
                .first()
                .expect("the file of a module maps back to that module");
            let analysis = if first == module {
                ModuleFileAnalysis::Analysed
            } else {
                ModuleFileAnalysis::AlreadyAnalysedAs(self.module_paths.path(first))
            };

            if file_modules.len() > 1 {
                graph.shared_module_files.insert(SharedModuleFile {
                    analysis: analysis.clone(),
                    file: file.clone(),
                    module: self.module_paths.path(module),
                });
            }

            if analysis == ModuleFileAnalysis::Analysed {
                graph.walked_files.insert(file);
                self.walk(
                    Subtree {
                        origin: SubtreeOrigin::SourceFile,
                        root: self.semantics.parse(source_file_id).syntax().clone(),
                    },
                    graph,
                );
            }
        }
    }

    fn expand(
        &self,
        call: &ast::MacroCall,
        origin: &SubtreeOrigin,
        pending: &mut Vec<Subtree>,
        CrateModuleGraph { violations, .. }: &mut CrateModuleGraph,
    ) {
        let database = self.semantics.db;
        let is_procedural = self
            .semantics
            .resolve_macro_call(call)
            .is_some_and(|definition| matches!(definition.kind(database), MacroKind::ProcMacro));

        if let Some(expansion) = self.semantics.to_def(call).filter(|_| !is_procedural) {
            let (parse, _) = &expansion.parse_macro_expansion(database).value;

            if !parse.errors().is_empty() && !self.token_origin.is_foreign_text(call.syntax()) {
                violations.insert(Violation {
                    kind: ViolationKind::UnparsableMacroExpansion,
                    location: self.locate(call.syntax(), origin),
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

    fn file_start(&self, file_id: FileId) -> SourceLocation {
        self.locator.locate(FileRangeWrapper {
            file_id,
            range: TextRange::empty(TextSize::from(0)),
        })
    }

    fn inactive(
        &self,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
        CrateModuleGraph {
            inactive_regions, ..
        }: &mut CrateModuleGraph,
    ) {
        if matches!(origin, SubtreeOrigin::SourceFile) {
            let original = self.semantics.original_range(node);
            let file_id = original.file_id.file_id(self.semantics.db);

            inactive_regions.insert(InactiveRegion {
                end: self.locator.locate(FileRangeWrapper {
                    file_id,
                    range: TextRange::empty(original.range.end()),
                }),
                start: self.locator.locate(FileRangeWrapper {
                    file_id,
                    range: TextRange::empty(original.range.start()),
                }),
            });
        }
    }

    #[expect(
        clippy::expect_used,
        reason = "rust-analyzer gives every node of a parsed module file a scope"
    )]
    fn is_constants_module(&self, container: &SyntaxNode) -> bool {
        ConstantsModule {
            database: self.semantics.db,
        }
        .matches(
            self.semantics
                .scope(container)
                .expect("the container belongs to a parsed module file")
                .module(),
        )
    }

    fn is_public_item_container(node: &SyntaxNode) -> bool {
        ast::SourceFile::can_cast(node.kind())
            || ast::ItemList::cast(node.clone()).is_some_and(|items| {
                items
                    .syntax()
                    .parent()
                    .is_some_and(|parent| ast::Module::can_cast(parent.kind()))
            })
    }

    fn is_reexport(&self, node: &SyntaxNode) -> bool {
        ast::Use::cast(node.clone())
            .and_then(|import| import.visibility())
            .or_else(|| {
                ast::ExternCrate::cast(node.clone())
                    .and_then(|declaration| declaration.visibility())
            })
            .is_some_and(|visibility| !self.token_origin.is_foreign_text(visibility.syntax()))
    }

    fn item_violations<'items>(
        &self,
        items: impl Iterator<Item = &'items ast::Item>,
        kind: &ViolationKind,
        origin: &SubtreeOrigin,
    ) -> Vec<Violation> {
        items
            .map(|item| Violation {
                kind: kind.clone(),
                location: self.locate(item.syntax(), origin),
            })
            .collect()
    }

    fn locate(&self, node: &SyntaxNode, origin: &SubtreeOrigin) -> SourceLocation {
        let database = self.semantics.db;
        let original = self.semantics.original_range(node);
        let file_id = original.file_id.file_id(database);

        match origin {
            SubtreeOrigin::MacroCall(outermost) if !self.token_origin.is_own_file(file_id) => {
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

    #[expect(
        clippy::expect_used,
        reason = "rust-analyzer defines every impl block of a module it has analysed"
    )]
    fn place(
        &self,
        implementation: &ast::Impl,
        origin: &SubtreeOrigin,
        CrateModuleGraph { violations, .. }: &mut CrateModuleGraph,
    ) {
        let database = self.semantics.db;
        let written_in = self
            .semantics
            .hir_file_for(implementation.syntax())
            .original_file(database)
            .file_id(database);
        let type_file = ImplementedType {
            database,
            implementation: self
                .semantics
                .to_def(implementation)
                .expect("the impl block belongs to an analysed module"),
            krate: self.krate,
        }
        .file();
        let location = self.locate(implementation.syntax(), origin);

        match type_file {
            Some(type_file) if type_file == written_in => {}
            Some(type_file) => {
                violations.insert(Violation {
                    kind: ViolationKind::MisplacedImpl {
                        type_file: self.file_start(type_file).file,
                    },
                    location,
                });
            }
            None => {
                violations.insert(Violation {
                    kind: ViolationKind::ImplWithoutLocalType,
                    location,
                });
            }
        }
    }

    fn record(
        &self,
        path: &ast::Path,
        origin: &SubtreeOrigin,
        CrateModuleGraph {
            dependencies,
            violations,
            ..
        }: &mut CrateModuleGraph,
    ) {
        match (PathReference {
            krate: self.krate,
            path,
            scope: match origin {
                SubtreeOrigin::MacroCall(outermost) => outermost.syntax(),
                SubtreeOrigin::SourceFile => path.syntax(),
            },
            semantics: self.semantics,
            token_origin: self.token_origin,
        })
        .outcome()
        {
            PathReferenceOutcome::Dependency(referenced) => {
                dependencies.insert(ModuleDependency {
                    from: self.module_paths.path(referenced.from),
                    location: self.locate(path.syntax(), origin),
                    to: self.module_paths.path(referenced.to),
                });
            }
            PathReferenceOutcome::Irrelevant => {}
            PathReferenceOutcome::Unresolved => {
                violations.insert(Violation {
                    kind: ViolationKind::UnresolvedPath,
                    location: self.locate(path.syntax(), origin),
                });
            }
        }
    }

    fn visit(
        &self,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
        pending: &mut Vec<Subtree>,
        graph: &mut CrateModuleGraph,
    ) {
        if let Some(path) = ast::Path::cast(node.clone()) {
            self.record(&path, origin, graph);
        } else if let Some(call) = ast::MacroCall::cast(node.clone()) {
            self.expand(&call, origin, pending, graph);
        } else if let Some(implementation) = ast::Impl::cast(node.clone())
            .filter(|implementation| !self.token_origin.is_foreign_text(implementation.syntax()))
        {
            self.place(&implementation, origin, graph);
        } else if self.is_reexport(node) {
            graph.violations.insert(Violation {
                kind: ViolationKind::Reexport,
                location: self.locate(node, origin),
            });
        } else if let Some(tree) = ast::UseTree::cast(node.clone()).filter(|tree| {
            tree.star_token().is_some_and(|star| {
                self.token_origin.is_own_text(
                    self.semantics.hir_file_for(tree.syntax()),
                    star.text_range(),
                )
            })
        }) {
            graph.violations.insert(Violation {
                kind: ViolationKind::GlobImport,
                location: self.locate(tree.syntax(), origin),
            });
        } else if let Some(constant) = ast::Const::cast(node.clone()).filter(|constant| {
            matches!(origin, SubtreeOrigin::SourceFile)
                && constant.visibility().is_some()
                && !constant.syntax().parent().is_some_and(|container| {
                    Self::is_public_item_container(&container)
                        && self.is_constants_module(&container)
                })
        }) {
            graph.violations.insert(Violation {
                kind: ViolationKind::PublicConstantOutsideConstantsModule,
                location: self.locate(constant.syntax(), origin),
            });
        } else if matches!(origin, SubtreeOrigin::SourceFile)
            && Self::is_public_item_container(node)
        {
            let non_constant = PublicItems {
                container: node,
                options: self.options,
                semantics: self.semantics,
            }
            .non_constant();

            if self.is_constants_module(node) {
                graph.violations.extend(self.item_violations(
                    non_constant.iter(),
                    &ViolationKind::NonConstantInConstantsModule,
                    origin,
                ));
            } else {
                graph.violations.extend(self.item_violations(
                    non_constant.iter().skip(1),
                    &ViolationKind::AdditionalPublicItem,
                    origin,
                ));
            }

            if self.crate_target_kind == CrateTargetKind::IntegrationTest {
                graph.violations.extend(
                    self.item_violations(
                        TestTargetItems {
                            container: node,
                            options: self.options,
                            semantics: self.semantics,
                        }
                        .neither_tests_constants_nor_imports()
                        .iter(),
                        &ViolationKind::NonTestItemInTestTarget,
                        origin,
                    ),
                );
            }
        }
    }

    fn walk(&self, root: Subtree, graph: &mut CrateModuleGraph) {
        let mut pending = vec![root];

        while let Some(subtree) = pending.pop() {
            let mut preorder = subtree.root.preorder();

            while let Some(event) = preorder.next() {
                if let WalkEvent::Enter(node) = event {
                    let configured = ConfiguredNode {
                        node: &node,
                        options: self.options,
                    };

                    graph.violations.extend(
                        configured
                            .negated_attributes()
                            .iter()
                            .filter(|attribute| {
                                !self.token_origin.is_foreign_text(attribute.syntax())
                            })
                            .map(|attribute| Violation {
                                kind: ViolationKind::NegatedConfiguration,
                                location: self.locate(attribute.syntax(), &subtree.origin),
                            }),
                    );

                    if configured.is_disabled() {
                        preorder.skip_subtree();
                        self.inactive(&node, &subtree.origin, graph);
                    } else {
                        self.visit(&node, &subtree.origin, &mut pending, graph);
                    }
                }
            }
        }
    }
}
