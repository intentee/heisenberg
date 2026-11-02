use std::collections::HashMap;

use ra_ap_cfg::CfgExpr;
use ra_ap_cfg::CfgOptions;
use ra_ap_hir::AsAssocItem;
use ra_ap_hir::AssocItemContainer;
use ra_ap_hir::Const;
use ra_ap_hir::Crate;
use ra_ap_hir::FileRangeWrapper;
use ra_ap_hir::HasVisibility;
use ra_ap_hir::HirFileId;
use ra_ap_hir::Impl;
use ra_ap_hir::InFile;
use ra_ap_hir::MacroKind;
use ra_ap_hir::Module;
use ra_ap_hir::ModuleId;
use ra_ap_hir::Semantics;
use ra_ap_hir_def::visibility::visibility_from_ast;
use ra_ap_hir_expand::Lookup;
use ra_ap_hir_expand::MacroDefKind;
use ra_ap_hir_expand::builtin::BuiltinFnLikeExpander;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::NodeOrToken;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::SyntaxNodePtr;
use ra_ap_syntax::TextRange;
use ra_ap_syntax::TextSize;
use ra_ap_syntax::WalkEvent;
use ra_ap_syntax::ast;
use ra_ap_syntax::ast::HasVisibility as _;
use ra_ap_vfs::FileId;

use heisenberg_module_graph::configured_out_negations::ConfiguredOutNegations;
use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_module_graph::crate_target_kind::CrateTargetKind;
use heisenberg_module_graph::inactive_region::InactiveRegion;
use heisenberg_module_graph::module_dependency::ModuleDependency;
use heisenberg_module_graph::module_file_analysis::ModuleFileAnalysis;
use heisenberg_module_graph::shared_module_file::SharedModuleFile;
use heisenberg_module_graph::source_location::SourceLocation;
use heisenberg_module_graph::violation::Violation;
use heisenberg_module_graph::violation_kind::ViolationKind;
use heisenberg_module_graph::walked_code::WalkedCode;

use crate::adt_violation::AdtViolation;
use crate::configured_node::ConfiguredNode;
use crate::constants_module::ConstantsModule;
use crate::implemented_type::ImplementedType;
use crate::is_module_inside_block::is_module_inside_block;
use crate::item_adt::ItemAdt;
use crate::item_definition::ItemDefinition;
use crate::item_expansion::ItemExpansion;
use crate::item_focus::ItemFocus;
use crate::lint_suppressions::LintSuppressions;
use crate::macro_expansion::MacroExpansion;
use crate::module_paths::ModulePaths;
use crate::negated_configuration::NegatedConfiguration;
use crate::outside_visibility::OutsideVisibility;
use crate::path_reference::PathReference;
use crate::path_reference_outcome::PathReferenceOutcome;
use crate::public_items::PublicItems;
use crate::source_locator::SourceLocator;
use crate::subtree::Subtree;
use crate::subtree_origin::SubtreeOrigin;
use crate::test_target_findings::TestTargetFindings;
use crate::test_target_items::TestTargetItems;
use crate::token_origin::TokenOrigin;

pub struct ModuleWalk<'walk, 'database> {
    pub crate_target_kind: CrateTargetKind,
    pub inactive_code: &'walk HashMap<InFile<SyntaxNodePtr>, CfgExpr>,
    pub krate: Crate,
    pub locator: &'walk SourceLocator<'walk>,
    pub module_paths: &'walk ModulePaths<'database>,
    pub options: &'walk CfgOptions,
    pub semantics: &'walk Semantics<'database, RootDatabase>,
    pub token_origin: &'walk TokenOrigin<'walk, 'database>,
}

impl ModuleWalk<'_, '_> {
    pub fn module(
        &self,
        module: Module,
        graph: &mut CrateModuleGraph,
        test_target: &mut TestTargetFindings,
    ) {
        let database = self.semantics.db;

        if let Some(source_file_id) = module.as_source_file_id(database) {
            let file_id = source_file_id.file_id(database);
            let file_modules: Vec<Module> = self.semantics.file_to_module_defs(file_id).collect();
            let file = self.file_start(file_id).file;
            let analysis = file_modules
                .first()
                .filter(|first| **first != module)
                .map_or(ModuleFileAnalysis::Analysed, |first| {
                    ModuleFileAnalysis::AlreadyAnalysedAs(self.module_paths.path(*first))
                });

            if file_modules.len() > 1 {
                graph.shared_module_files.insert(SharedModuleFile {
                    analysis: analysis.clone(),
                    file: file.clone(),
                    module: self.module_paths.path(module),
                });
            }

            if analysis == ModuleFileAnalysis::Analysed {
                graph.walked_code.files.insert(file);
                self.walk(
                    Subtree {
                        origin: SubtreeOrigin::SourceFile,
                        root: self.semantics.parse(source_file_id).syntax().clone(),
                    },
                    graph,
                    test_target,
                );
            }
        }
    }

    fn declare(
        &self,
        declaration: &ast::Module,
        module: Module,
        origin: &SubtreeOrigin,
        CrateModuleGraph {
            configured_out_negations: ConfiguredOutNegations { declarations, .. },
            violations,
            ..
        }: &mut CrateModuleGraph,
    ) {
        let database = self.semantics.db;

        if is_module_inside_block(database, module) {
            violations.insert(Violation {
                kind: ViolationKind::ModuleFileInsideBlock,
                location: self.locate_focus(declaration.syntax(), origin),
            });
        } else if let Some(source_file_id) = module.as_source_file_id(database)
            && !(ConfiguredNode {
                node: self.semantics.parse(source_file_id).syntax(),
                options: self.options,
            })
            .negated_attributes()
            .is_empty()
        {
            declarations.insert(self.locate_focus(declaration.syntax(), origin));
        }
    }

    fn expand(
        &self,
        call: &ast::MacroCall,
        origin: &SubtreeOrigin,
        pending: &mut Vec<Subtree>,
        graph: &mut CrateModuleGraph,
    ) {
        let database = self.semantics.db;

        if let Some(expansion) = self.semantics.to_def(call) {
            if matches!(
                expansion.lookup(database).def.kind,
                MacroDefKind::BuiltIn(_, BuiltinFnLikeExpander::CfgSelect)
            ) && !self.token_origin.is_foreign_text(call.syntax())
            {
                graph.violations.insert(Violation {
                    kind: ViolationKind::ConfigurationSelectionMacro,
                    location: self.locate_focus(call.syntax(), origin),
                });
            }

            if expansion.kind(database) != MacroKind::ProcMacro {
                self.push_expansion(
                    HirFileId::from(expansion),
                    call.syntax(),
                    origin,
                    pending,
                    graph,
                );
            }
        }
    }

    fn expand_derives(
        &self,
        derive: &ast::Meta,
        origin: &SubtreeOrigin,
        pending: &mut Vec<Subtree>,
        graph: &mut CrateModuleGraph,
    ) {
        for expansion in self
            .semantics
            .expand_derive_macro(derive)
            .into_iter()
            .flatten()
            .flatten()
        {
            self.push_expansion(
                self.semantics.hir_file_for(&expansion.value),
                derive.syntax(),
                origin,
                pending,
                graph,
            );
        }
    }

    fn file_start(&self, file_id: FileId) -> SourceLocation {
        self.locator.locate(FileRangeWrapper {
            file_id,
            range: TextRange::empty(TextSize::from(0)),
        })
    }

    fn implemented_type(&self, implementation: &ast::Impl) -> Option<ImplementedType<'_>> {
        self.semantics
            .to_def(implementation)
            .map(|implementation| ImplementedType {
                database: self.semantics.db,
                implementation,
                krate: self.krate,
            })
    }

    fn inactive(
        &self,
        node: &SyntaxNode,
        CrateModuleGraph {
            walked_code: WalkedCode {
                inactive_regions, ..
            },
            ..
        }: &mut CrateModuleGraph,
    ) {
        let file = self.semantics.hir_file_for(node);
        let focus = self.token_origin.written_range(
            file,
            ItemFocus {
                node,
                semantics: self.semantics,
            }
            .range(),
        );

        if self.token_origin.is_own_file(focus.file_id) {
            let covered = node
                .descendants_with_tokens()
                .filter_map(NodeOrToken::into_token)
                .filter(|token| !token.kind().is_trivia())
                .map(|token| self.token_origin.written_range(file, token.text_range()))
                .filter(|written| written.file_id == focus.file_id)
                .fold(focus.range, |covered, written| covered.cover(written.range));

            inactive_regions.insert(InactiveRegion {
                end: self.locator.locate(FileRangeWrapper {
                    file_id: focus.file_id,
                    range: TextRange::empty(covered.end()),
                }),
                focus: self.locator.locate(focus),
                start: self.locator.locate(FileRangeWrapper {
                    file_id: focus.file_id,
                    range: TextRange::empty(covered.start()),
                }),
            });
        }
    }

    fn is_constants_module(&self, container: &SyntaxNode) -> bool {
        self.semantics.scope(container).is_some_and(|scope| {
            ConstantsModule {
                database: self.semantics.db,
            }
            .matches(scope.module())
        })
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

    fn is_reexport(&self, node: &SyntaxNode, definition: &ItemDefinition) -> bool {
        let database = self.semantics.db;
        let outside = OutsideVisibility { database };

        !self.token_origin.is_foreign_text(node)
            && (ast::Use::cast(node.clone()).is_some_and(|import| {
                self.semantics.scope(import.syntax()).is_some_and(|scope| {
                    let module = scope.module();

                    outside.reaches_outside(
                        visibility_from_ast(
                            database,
                            ModuleId::from(module),
                            InFile::new(
                                self.semantics.hir_file_for(import.syntax()),
                                import.visibility(),
                            ),
                        ),
                        module,
                    )
                })
            }) || matches!(definition, ItemDefinition::ExternCrate(declaration) if outside.reaches_outside(
                declaration.visibility(database),
                declaration.module(database),
            )))
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
                location: self.locate_focus(item.syntax(), origin),
            })
            .collect()
    }

    fn locate(&self, node: &SyntaxNode, origin: &SubtreeOrigin) -> SourceLocation {
        self.locate_original(
            self.semantics
                .diagnostics_display_range_for_range(InFile::new(
                    self.semantics.hir_file_for(node),
                    node.text_range(),
                )),
            origin,
        )
    }

    fn locate_focus(&self, node: &SyntaxNode, origin: &SubtreeOrigin) -> SourceLocation {
        self.locate_original(
            ItemFocus {
                node,
                semantics: self.semantics,
            }
            .original_range(),
            origin,
        )
    }

    fn locate_original(
        &self,
        original: FileRangeWrapper<FileId>,
        origin: &SubtreeOrigin,
    ) -> SourceLocation {
        match origin {
            SubtreeOrigin::MacroCall(outermost)
                if !self.token_origin.is_own_file(original.file_id) =>
            {
                let call = self.semantics.original_range(outermost);

                self.locator.locate(FileRangeWrapper {
                    file_id: call.file_id.file_id(self.semantics.db),
                    range: call.range,
                })
            }
            SubtreeOrigin::MacroCall(_) | SubtreeOrigin::SourceFile => {
                self.locator.locate(original)
            }
        }
    }

    fn misplaced_constant(&self, definition: Const) -> Option<ViolationKind> {
        let database = self.semantics.db;
        let module = definition.module(database);
        let reaches_outside =
            OutsideVisibility { database }.reaches_outside(definition.visibility(database), module);

        match definition
            .as_assoc_item(database)
            .map(|item| item.container(database))
        {
            Some(AssocItemContainer::Trait(_)) => None,
            Some(AssocItemContainer::Impl(implementation)) => {
                (implementation.trait_(database).is_none() && reaches_outside)
                    .then_some(ViolationKind::PublicAssociatedConstant)
            }
            None => (reaches_outside && !ConstantsModule { database }.matches(module))
                .then_some(ViolationKind::PublicConstantOutsideConstantsModule),
        }
    }

    fn misplacement(
        &self,
        implementation: &ast::Impl,
        definition: Impl,
        origin: &SubtreeOrigin,
    ) -> Option<Violation> {
        let database = self.semantics.db;
        let written_in = self
            .semantics
            .hir_file_for(implementation.syntax())
            .original_file(database)
            .file_id(database);
        let location = self.locate_focus(implementation.syntax(), origin);

        match (ImplementedType {
            database,
            implementation: definition,
            krate: self.krate,
        })
        .file()
        {
            Some(type_file) if type_file == written_in => None,
            Some(type_file) => Some(Violation {
                kind: ViolationKind::MisplacedImpl {
                    type_file: self.file_start(type_file).file,
                },
                location,
            }),
            None => Some(Violation {
                kind: ViolationKind::ImplWithoutLocalType,
                location,
            }),
        }
    }

    fn push_expansion(
        &self,
        expansion: HirFileId,
        call: &SyntaxNode,
        origin: &SubtreeOrigin,
        pending: &mut Vec<Subtree>,
        CrateModuleGraph { violations, .. }: &mut CrateModuleGraph,
    ) {
        if (MacroExpansion {
            database: self.semantics.db,
            file: expansion,
        })
        .is_parsable()
        {
            pending.push(Subtree {
                origin: origin.expansion_of(call),
                root: self.semantics.parse_or_expand(expansion),
            });
        } else {
            violations.insert(Violation {
                kind: ViolationKind::UnparsableMacroExpansion,
                location: self.locate(call, origin),
            });
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
            module_paths: self.module_paths,
            path,
            scope: match origin {
                SubtreeOrigin::MacroCall(outermost) => outermost,
                SubtreeOrigin::SourceFile => path.syntax(),
            },
            semantics: self.semantics,
            token_origin: self.token_origin,
        })
        .outcome()
        {
            PathReferenceOutcome::Dependency(referenced) => {
                dependencies.insert(ModuleDependency {
                    from: referenced.from,
                    location: self.locate(path.syntax(), origin),
                    to: referenced.to,
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

    fn test_target_violations(
        &self,
        items: &[ast::Item],
        origin: &SubtreeOrigin,
        CrateModuleGraph { violations, .. }: &mut CrateModuleGraph,
        TestTargetFindings { impl_blocks, types }: &mut TestTargetFindings,
    ) {
        for item in items {
            let violation = Violation {
                kind: ViolationKind::NonTestItemInTestTarget,
                location: self.locate_focus(item.syntax(), origin),
            };

            if let Some(adt) = (ItemAdt {
                item,
                semantics: self.semantics,
            })
            .adt()
            {
                types.push(AdtViolation {
                    adt,
                    violation: violation.clone(),
                });
                violations.insert(violation);
            } else if let Some(adt) = ast::Impl::cast(item.syntax().clone())
                .and_then(|implementation| self.implemented_type(&implementation))
                .and_then(|implemented| implemented.adt())
            {
                impl_blocks.push(AdtViolation { adt, violation });
            } else {
                violations.insert(violation);
            }
        }
    }

    fn visit(
        &self,
        node: &SyntaxNode,
        definition: &ItemDefinition,
        origin: &SubtreeOrigin,
        pending: &mut Vec<Subtree>,
        graph: &mut CrateModuleGraph,
        test_target: &mut TestTargetFindings,
    ) {
        if let Some(path) = ast::Path::cast(node.clone()) {
            self.record(&path, origin, graph);
        } else if let Some(call) = ast::MacroCall::cast(node.clone()) {
            self.expand(&call, origin, pending, graph);
        } else if let Some(derive) = ast::Meta::cast(node.clone()) {
            self.expand_derives(&derive, origin, pending, graph);
        } else if let ItemDefinition::Module(module) = definition
            && let Some(declaration) = ast::Module::cast(node.clone())
                .filter(|declaration| declaration.item_list().is_none())
        {
            self.declare(&declaration, *module, origin, graph);
        } else if let ItemDefinition::Impl(implementation) = definition
            && let Some(written) = ast::Impl::cast(node.clone())
                .filter(|written| !self.token_origin.is_foreign_text(written.syntax()))
        {
            graph
                .violations
                .extend(self.misplacement(&written, *implementation, origin));
        } else if self.is_reexport(node, definition) {
            graph.violations.insert(Violation {
                kind: ViolationKind::Reexport,
                location: self.locate_focus(node, origin),
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
        } else if matches!(origin, SubtreeOrigin::SourceFile)
            && let ItemDefinition::Const(constant) = definition
            && let Some(kind) = self.misplaced_constant(*constant)
        {
            graph.violations.insert(Violation {
                kind,
                location: self.locate_focus(node, origin),
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
                self.test_target_violations(
                    &TestTargetItems {
                        container: node,
                        options: self.options,
                        semantics: self.semantics,
                    }
                    .neither_tests_constants_nor_imports(),
                    origin,
                    graph,
                    test_target,
                );
            }
        }
    }

    fn walk(
        &self,
        root: Subtree,
        graph: &mut CrateModuleGraph,
        test_target: &mut TestTargetFindings,
    ) {
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

                    let predicate = self.inactive_code.get(&InFile::new(
                        self.semantics.hir_file_for(&node),
                        SyntaxNodePtr::new(&node),
                    ));

                    if configured.is_disabled() {
                        preorder.skip_subtree();
                        self.inactive(&node, graph);
                    } else if let Some(expression) = predicate {
                        preorder.skip_subtree();

                        if (NegatedConfiguration { expression }).is_negated() {
                            graph.configured_out_negations.negations.insert(Violation {
                                kind: ViolationKind::NegatedConfiguration,
                                location: self.locate_focus(&node, &subtree.origin),
                            });
                        }

                        self.inactive(&node, graph);
                    } else {
                        graph.violations.extend(
                            LintSuppressions {
                                node: &node,
                                options: self.options,
                                semantics: self.semantics,
                            }
                            .attributes()
                            .iter()
                            .filter(|meta| self.token_origin.is_workspace_text(meta.syntax()))
                            .map(|meta| Violation {
                                kind: ViolationKind::LintSuppression,
                                location: self.locate(meta.syntax(), &subtree.origin),
                            }),
                        );

                        match ast::Item::cast(node.clone())
                            .map_or(ItemExpansion::Unexpanded, |item| {
                                ItemExpansion::new(&item, self.semantics)
                            }) {
                            ItemExpansion::Parsable { generated, written } => {
                                preorder.skip_subtree();
                                pending.extend(written.iter().map(|item| Subtree {
                                    origin: subtree.origin.clone(),
                                    root: item.syntax().clone(),
                                }));
                                pending.extend(generated.iter().map(|item| Subtree {
                                    origin: subtree.origin.expansion_of(&node),
                                    root: item.syntax().clone(),
                                }));
                            }
                            ItemExpansion::Unexpanded => match ast::Item::cast(node.clone())
                                .map_or(ItemDefinition::NotDefining, |item| {
                                    ItemDefinition::new(&item, self.semantics)
                                }) {
                                ItemDefinition::Unmapped => {
                                    preorder.skip_subtree();
                                    graph.violations.insert(Violation {
                                        kind: ViolationKind::UnmappedItem,
                                        location: self.locate_focus(&node, &subtree.origin),
                                    });
                                }
                                definition => {
                                    self.visit(
                                        &node,
                                        &definition,
                                        &subtree.origin,
                                        &mut pending,
                                        graph,
                                        test_target,
                                    );
                                }
                            },
                            ItemExpansion::Unparsable => {
                                preorder.skip_subtree();
                                graph.violations.insert(Violation {
                                    kind: ViolationKind::UnparsableMacroExpansion,
                                    location: self.locate_focus(&node, &subtree.origin),
                                });
                            }
                        }
                    }
                }
            }
        }
    }
}
