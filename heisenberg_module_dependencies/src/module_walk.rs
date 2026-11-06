use std::collections::BTreeSet;
use std::collections::HashMap;
use std::iter;

use ra_ap_cfg::CfgExpr;
use ra_ap_cfg::CfgOptions;
use ra_ap_hir::Adt;
use ra_ap_hir::AsAssocItem;
use ra_ap_hir::AssocItemContainer;
use ra_ap_hir::Const;
use ra_ap_hir::Crate;
use ra_ap_hir::ExpandResult;
use ra_ap_hir::FileRangeWrapper;
use ra_ap_hir::Function;
use ra_ap_hir::HasVisibility;
use ra_ap_hir::HirDisplay as _;
use ra_ap_hir::HirFileId;
use ra_ap_hir::Impl;
use ra_ap_hir::InFile;
use ra_ap_hir::MacroCallId;
use ra_ap_hir::Module;
use ra_ap_hir::ModuleDef;
use ra_ap_hir::ModuleId;
use ra_ap_hir::Semantics;
use ra_ap_hir_def::visibility::visibility_from_ast;
use ra_ap_hir_expand::Lookup;
use ra_ap_hir_expand::MacroDefKind;
use ra_ap_hir_expand::builtin::BuiltinFnLikeExpander;
use ra_ap_hir_expand::name::AsName as _;
use ra_ap_hir_expand::name::Name;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::defs::Definition;
use ra_ap_ide_db::search::FileReference;
use ra_ap_paths::AbsPath;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::AstToken;
use ra_ap_syntax::NodeOrToken;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::SyntaxNodePtr;
use ra_ap_syntax::SyntaxToken;
use ra_ap_syntax::TextRange;
use ra_ap_syntax::TextSize;
use ra_ap_syntax::WalkEvent;
use ra_ap_syntax::ast;
use ra_ap_syntax::ast::HasName as _;
use ra_ap_syntax::ast::HasVisibility as _;
use ra_ap_vfs::FileId;

use heisenberg_module_graph::configured_out_negations::ConfiguredOutNegations;
use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_module_graph::crate_target::CrateTarget;
use heisenberg_module_graph::crate_target_kind::CrateTargetKind;
use heisenberg_module_graph::foreign_module::ForeignModule;
use heisenberg_module_graph::inactive_focus::InactiveFocus;
use heisenberg_module_graph::item_group::ItemGroup;
use heisenberg_module_graph::item_placement::ItemPlacement;
use heisenberg_module_graph::method_placement::MethodPlacement;
use heisenberg_module_graph::module_dependency::ModuleDependency;
use heisenberg_module_graph::module_file_analysis::ModuleFileAnalysis;
use heisenberg_module_graph::shared_module_file::SharedModuleFile;
use heisenberg_module_graph::source_location::SourceLocation;
use heisenberg_module_graph::unresolved_foreign_reference::UnresolvedForeignReference;
use heisenberg_module_graph::violation::Violation;
use heisenberg_module_graph::violation_kind::ViolationKind;
use heisenberg_module_graph::violation_set::ViolationSet;
use heisenberg_module_graph::walked_code::WalkedCode;
use heisenberg_module_graph::written_tokens::WrittenTokens;

use crate::adt_violation::AdtViolation;
use crate::ancestry::Ancestry;
use crate::code_tokens::code_tokens;
use crate::configured_node::ConfiguredNode;
use crate::constants::LIFETIME_PREFIX;
use crate::constants_module::ConstantsModule;
use crate::constructor::Constructor;
use crate::expansion_chain::ExpansionChain;
use crate::field_only_parameter::FieldOnlyParameter;
use crate::file_local_item::FileLocalItem;
use crate::impl_home::ImplHome;
use crate::implemented_type::ImplementedType;
use crate::is_module_inside_block::is_module_inside_block;
use crate::item_adt::ItemAdt;
use crate::item_definition::ItemDefinition;
use crate::item_expansion::ItemExpansion;
use crate::item_focus::ItemFocus;
use crate::item_order::ItemOrder;
use crate::lint_suppressions::LintSuppressions;
use crate::local_impl_home::LocalImplHome;
use crate::macro_expansion::MacroExpansion;
use crate::method_key::MethodKey;
use crate::method_order::MethodOrder;
use crate::misordered_entry::MisorderedEntry;
use crate::module_paths::ModulePaths;
use crate::name_words::NameWords;
use crate::negated_configuration::NegatedConfiguration;
use crate::ordered_entries::OrderedEntries;
use crate::ordered_entry::OrderedEntry;
use crate::original_file_id::original_file_id;
use crate::outside_visibility::OutsideVisibility;
use crate::parameter_name::ParameterName;
use crate::path_reference::PathReference;
use crate::path_reference_outcome::PathReferenceOutcome;
use crate::public_items::PublicItems;
use crate::signature_tuple::SignatureTuple;
use crate::source_locator::SourceLocator;
use crate::subtree::Subtree;
use crate::subtree_origin::SubtreeOrigin;
use crate::test_code::TestCode;
use crate::test_target_findings::TestTargetFindings;
use crate::test_target_items::TestTargetItems;
use crate::token_origin::TokenOrigin;
use crate::trait_arguments::trait_arguments;

pub struct ModuleWalk<'walk, 'database> {
    pub crate_target: &'walk CrateTarget,
    pub inactive_code: &'walk HashMap<InFile<SyntaxNodePtr>, CfgExpr>,
    pub krate: Crate,
    pub locator: &'walk SourceLocator<'walk>,
    pub module_paths: &'walk ModulePaths<'database>,
    pub options: &'walk CfgOptions,
    pub production_options: &'walk CfgOptions,
    pub semantics: &'walk Semantics<'database, RootDatabase>,
    pub test_code: &'walk TestCode<'walk, 'database>,
    pub token_origin: &'walk TokenOrigin<'walk, 'database>,
    pub unpublished_packages: &'walk BTreeSet<String>,
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
                let root = self.semantics.parse(source_file_id).syntax().clone();

                graph.walked_code.files.insert(file);
                self.non_doc_comments(file_id, &root, graph);
                self.walk(
                    Subtree {
                        origin: SubtreeOrigin::SourceFile,
                        root,
                    },
                    graph,
                    test_target,
                );
            }
        }
    }

    fn activate(
        &self,
        node: &SyntaxNode,
        CrateModuleGraph {
            walked_code: WalkedCode { active_tokens, .. },
            ..
        }: &mut CrateModuleGraph,
    ) {
        if !ast::TokenTree::can_cast(node.kind()) {
            let tokens: Vec<SyntaxToken> = code_tokens(node.children_with_tokens()).collect();

            self.record_own_tokens(self.semantics.hir_file_for(node), &tokens, active_tokens);
        }
    }

    fn container(
        &self,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
        graph: &mut CrateModuleGraph,
        test_target: &mut TestTargetFindings,
    ) {
        let non_constant = PublicItems {
            container: node,
            options: self.options,
            semantics: self.semantics,
        }
        .non_constant();

        graph
            .violations
            .extend(self.item_order_violations(node, origin));

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
            graph.violations.extend(self.module_naming_violation(
                node,
                non_constant.first(),
                origin,
            ));
        }

        if self.crate_target.kind == CrateTargetKind::IntegrationTest {
            self.test_target_violations(
                &TestTargetItems {
                    container: node,
                    options: self.options,
                    semantics: self.semantics,
                    test_harness: self.crate_target.test_harness,
                }
                .neither_tests_constants_nor_imports(),
                origin,
                graph,
                test_target,
            );
        }
    }

    fn contains_unresolved_reference(&self, node: &SyntaxNode) -> bool {
        node.descendants().any(|descendant| {
            ast::Path::cast(descendant.clone())
                .is_some_and(|path| self.semantics.resolve_path(&path).is_none())
                || ast::MethodCallExpr::cast(descendant)
                    .is_some_and(|call| self.is_unresolved_method_call(&call))
        })
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
            let call_location = expansion.lookup(database);

            if let Some(included) = call_location.include_file_id(database, expansion) {
                self.non_doc_comments(
                    included.file_id(database),
                    self.semantics.parse(included).syntax(),
                    graph,
                );
            }

            if matches!(
                call_location.def.kind,
                MacroDefKind::BuiltIn(_, BuiltinFnLikeExpander::CfgSelect)
            ) && !self.token_origin.is_foreign_text(call.syntax())
            {
                graph.violations.insert(Violation {
                    kind: ViolationKind::ConfigurationSelectionMacro,
                    location: self.locate_focus(call.syntax(), origin),
                });
            }

            pending.extend(expansion.eager_arg(database).map(|argument| Subtree {
                origin: origin.expansion_of(call.syntax()),
                root: self.semantics.parse_or_expand(HirFileId::from(argument)),
            }));

            self.push_expansion(expansion, call.syntax(), origin, pending, graph);
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
            .filter_map(|ExpandResult { value, .. }| {
                self.semantics.hir_file_for(&value).macro_file()
            })
        {
            self.push_expansion(expansion, derive.syntax(), origin, pending, graph);
        }
    }

    fn file_start(&self, file_id: FileId) -> SourceLocation {
        self.locator.locate(FileRangeWrapper {
            file_id,
            range: TextRange::empty(TextSize::from(0)),
        })
    }

    fn function_used_only_by_tests(
        &self,
        function: Function,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
    ) -> Option<Violation> {
        let database = self.semantics.db;

        (!function
            .as_assoc_item(database)
            .is_some_and(|item| match item.container(database) {
                AssocItemContainer::Trait(_) => true,
                AssocItemContainer::Impl(implementation) => {
                    implementation.trait_(database).is_some()
                }
            })
            && (self
                .unpublished_packages
                .contains(&self.crate_target.package_name)
                || !self.is_reachable_from_other_crates(function))
            && !self.test_code.contains(node)
            && !self.in_test_gated_code(node)
            && {
                let references = Definition::Function(function)
                    .usages(self.semantics)
                    .all()
                    .references;

                !references.is_empty()
                    && references
                        .values()
                        .flatten()
                        .all(|FileReference { name, .. }| {
                            name.syntax()
                                .parent()
                                .is_some_and(|parent| self.test_code.contains(&parent))
                        })
            })
        .then(|| Violation {
            kind: ViolationKind::FunctionUsedOnlyByTests,
            location: self.locate_focus(node, origin),
        })
    }

    fn generic_parameter_violation(
        &self,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
    ) -> Option<Violation> {
        ast::LifetimeParam::cast(node.clone())
            .and_then(|parameter| parameter.lifetime())
            .and_then(|lifetime| {
                Name::new_lifetime(lifetime.text())
                    .as_str()
                    .strip_prefix(LIFETIME_PREFIX)
                    .and_then(|identifier| ParameterName { identifier }.single_letter_violation())
                    .and_then(|kind| self.own_violation(kind, lifetime.syntax(), origin))
            })
            .or_else(|| {
                ast::TypeParam::cast(node.clone())
                    .and_then(|parameter| parameter.name())
                    .and_then(|name| {
                        ParameterName {
                            identifier: name.as_name().as_str(),
                        }
                        .type_parameter_violation()
                        .and_then(|kind| self.own_violation(kind, name.syntax(), origin))
                    })
            })
            .or_else(|| {
                ast::ConstParam::cast(node.clone())
                    .and_then(|parameter| parameter.name())
                    .and_then(|name| {
                        ParameterName {
                            identifier: name.as_name().as_str(),
                        }
                        .single_letter_violation()
                        .and_then(|kind| self.own_violation(kind, name.syntax(), origin))
                    })
            })
    }

    fn has_own_visibility(&self, visibility: Option<ast::Visibility>) -> bool {
        visibility.is_some_and(|visibility| !self.token_origin.is_foreign_text(visibility.syntax()))
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

    fn in_test_gated_code(&self, node: &SyntaxNode) -> bool {
        Ancestry {
            node,
            semantics: self.semantics,
        }
        .nodes()
        .iter()
        .any(|enclosing| self.is_test_gated(enclosing))
    }

    fn inactive(
        &self,
        node: &SyntaxNode,
        negations: BTreeSet<SourceLocation>,
        CrateModuleGraph {
            walked_code:
                WalkedCode {
                    inactive_foci,
                    inactive_tokens,
                    ..
                },
            ..
        }: &mut CrateModuleGraph,
    ) {
        let file = self.semantics.hir_file_for(node);
        let focus_range = ItemFocus {
            node,
            semantics: self.semantics,
        }
        .range();
        let focus = self.token_origin.written_range(file, focus_range);
        let tokens: Vec<SyntaxToken> = code_tokens(node.descendants_with_tokens()).collect();

        self.record_own_tokens(file, &tokens, inactive_tokens);

        if self.token_origin.is_own_text(file, focus_range) {
            inactive_foci.insert(InactiveFocus {
                location: self.locator.locate(focus),
                negations,
                token_starts: tokens
                    .iter()
                    .filter(|token| focus_range.contains_range(token.text_range()))
                    .map(|token| self.token_origin.written_range(file, token.text_range()))
                    .filter(|FileRangeWrapper { file_id, .. }| *file_id == focus.file_id)
                    .map(|FileRangeWrapper { range, .. }| u32::from(range.start()))
                    .collect(),
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

    fn is_exempt_from_the_procedural_rule(&self, function: Function, node: &SyntaxNode) -> bool {
        let database = self.semantics.db;

        function.self_param(database).is_some()
            || function.as_assoc_item(database).is_some_and(|item| {
                matches!(
                    item.container(database),
                    AssocItemContainer::Impl(implementation)
                        if implementation.trait_(database).is_some()
                            || Constructor {
                                function,
                                krate: self.krate,
                                semantics: self.semantics,
                            }
                            .constructs(&implementation.self_ty(database))
                )
            })
            || FileLocalItem {
                definition: ModuleDef::Function(function),
                file_id: original_file_id(database, self.semantics.hir_file_for(node)),
                semantics: self.semantics,
            }
            .is_file_local()
    }

    fn is_own_crate_adt(&self, adt: Adt) -> bool {
        adt.module(self.semantics.db).krate(self.semantics.db) == self.krate
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

    fn is_reachable_from_other_crates(&self, function: Function) -> bool {
        let database = self.semantics.db;

        function.visibility(database).is_public()
            && function
                .module(database)
                .path_to_root(database)
                .iter()
                .all(|module| {
                    module.is_crate_root(database) || module.visibility(database).is_public()
                })
            && !function.as_assoc_item(database).is_some_and(|item| {
                matches!(
                    item.container(database),
                    AssocItemContainer::Impl(implementation)
                        if implementation
                            .self_ty(database)
                            .as_adt()
                            .is_some_and(|adt| !adt.visibility(database).is_public())
                )
            })
    }

    fn is_reexport(&self, node: &SyntaxNode, definition: &ItemDefinition) -> bool {
        let database = self.semantics.db;
        let outside = OutsideVisibility { database };

        ast::Use::cast(node.clone()).is_some_and(|import| {
            self.has_own_visibility(import.visibility())
                && self.semantics.scope(import.syntax()).is_some_and(|scope| {
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
        }) || (ast::ExternCrate::cast(node.clone())
            .is_some_and(|written| self.has_own_visibility(written.visibility()))
            && matches!(definition, ItemDefinition::ExternCrate(declaration) if outside.reaches_outside(
                declaration.visibility(database),
                declaration.module(database),
            )))
    }

    fn is_test_gated(&self, node: &SyntaxNode) -> bool {
        ConfiguredNode {
            node,
            options: self.production_options,
        }
        .is_disabled()
    }

    fn is_unresolved_method_call(&self, call: &ast::MethodCallExpr) -> bool {
        self.semantics.resolve_method_call(call).is_none()
    }

    fn item_order_violations(
        &self,
        container: &SyntaxNode,
        origin: &SubtreeOrigin,
    ) -> Vec<Violation> {
        self.order_violations(
            ItemOrder {
                container,
                krate: self.krate,
                options: self.options,
                semantics: self.semantics,
                workspace_files: self.token_origin.workspace_files,
            }
            .entries(),
            |group: &ItemGroup, preceding_group: &ItemGroup, preceding| {
                ViolationKind::ItemOutOfOrder {
                    placement: ItemPlacement {
                        group: *group,
                        preceding,
                        preceding_group: *preceding_group,
                    },
                }
            },
            origin,
        )
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

    fn lint_suppressions(
        &self,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
        CrateModuleGraph { violations, .. }: &mut CrateModuleGraph,
    ) {
        violations.extend(
            LintSuppressions {
                node,
                options: self.options,
                semantics: self.semantics,
            }
            .attributes()
            .iter()
            .filter(|meta| self.token_origin.is_workspace_text(meta.syntax()))
            .map(|meta| Violation {
                kind: ViolationKind::LintSuppression,
                location: self.locate(meta.syntax(), origin),
            }),
        );
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

    fn method_order_violations(
        &self,
        implementation: &ast::Impl,
        origin: &SubtreeOrigin,
    ) -> Vec<Violation> {
        self.order_violations(
            MethodOrder {
                implementation,
                krate: self.krate,
                options: self.options,
                semantics: self.semantics,
            }
            .entries(),
            |MethodKey { group, .. }: &MethodKey,
             MethodKey {
                 group: preceding_group,
                 ..
             }: &MethodKey,
             preceding| ViolationKind::MethodOutOfOrder {
                placement: MethodPlacement {
                    group: *group,
                    preceding,
                    preceding_group: *preceding_group,
                },
            },
            origin,
        )
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

    fn misplaced_impl_kind(&self, home: &LocalImplHome, file: FileId) -> ViolationKind {
        let database = self.semantics.db;
        let home_file = self.locator.file(file);

        match home {
            LocalImplHome::Type { .. } | LocalImplHome::TypeArgument { .. } => {
                ViolationKind::MisplacedImpl {
                    type_file: home_file,
                }
            }
            LocalImplHome::TraitBlanket { .. } => ViolationKind::MisplacedBlanketImpl {
                trait_file: home_file,
            },
            LocalImplHome::TraitObject { .. } => ViolationKind::MisplacedTraitObjectImpl {
                trait_file: home_file,
            },
            LocalImplHome::TraitForeignType { owner } => ViolationKind::MisplacedForeignTypeImpl {
                trait_file: home_file,
                trait_name: owner
                    .name(database)
                    .display(database, self.krate.edition(database))
                    .to_string(),
            },
        }
    }

    fn misplacement(
        &self,
        implementation: &ast::Impl,
        definition: Impl,
        origin: &SubtreeOrigin,
    ) -> Option<Violation> {
        let database = self.semantics.db;
        let written_in = original_file_id(
            database,
            self.semantics.hir_file_for(implementation.syntax()),
        );
        let location = self.locate_focus(implementation.syntax(), origin);

        match (ImplementedType {
            database,
            implementation: definition,
            krate: self.krate,
        })
        .home(&trait_arguments(implementation, self.semantics))
        {
            ImplHome::Foreign => Some(Violation {
                kind: ViolationKind::ImplWithoutLocalType,
                location,
            }),
            ImplHome::Local { home } => {
                home.file(database)
                    .filter(|file| *file != written_in)
                    .map(|file| Violation {
                        kind: self.misplaced_impl_kind(&home, file),
                        location,
                    })
            }
        }
    }

    fn module_name(&self, module: Module) -> Option<String> {
        let database = self.semantics.db;

        module.name(database).and_then(|name| {
            if module.is_inline(database) {
                Some(name.as_str().to_owned())
            } else {
                self.locator
                    .vfs
                    .file_path(original_file_id(
                        database,
                        module.definition_source_file_id(database),
                    ))
                    .as_path()
                    .and_then(|path| {
                        if module.is_mod_rs(database) {
                            path.parent().and_then(AbsPath::file_name)
                        } else {
                            path.file_stem()
                        }
                    })
                    .map(str::to_owned)
            }
        })
    }

    fn module_naming_violation(
        &self,
        container: &SyntaxNode,
        public_item: Option<&ast::Item>,
        origin: &SubtreeOrigin,
    ) -> Option<Violation> {
        public_item.and_then(|item| {
            self.semantics
                .scope(container)
                .and_then(|scope| self.module_name(scope.module()))
                .and_then(|module| {
                    ast::AnyHasName::cast(item.syntax().clone())
                        .and_then(|named| named.name())
                        .and_then(|name| {
                            let text = name.text();
                            let words = NameWords { name: text };

                            (!words.are_spelled_by(&module)).then(|| Violation {
                                kind: ViolationKind::ModuleNotNamedAfterItsItem {
                                    expected: words.snake_case(),
                                    item: text.to_owned(),
                                    module,
                                },
                                location: self.locate_focus(item.syntax(), origin),
                            })
                        })
                })
        })
    }

    fn negated_attributes(
        &self,
        configured: &ConfiguredNode<'_>,
        origin: &SubtreeOrigin,
        CrateModuleGraph { violations, .. }: &mut CrateModuleGraph,
    ) -> BTreeSet<SourceLocation> {
        let negations: BTreeSet<SourceLocation> = configured
            .negated_attributes()
            .iter()
            .filter(|attribute| !self.token_origin.is_foreign_text(attribute.syntax()))
            .map(|attribute| self.locate(attribute.syntax(), origin))
            .collect();

        violations.extend(negations.iter().map(|location| Violation {
            kind: ViolationKind::NegatedConfiguration,
            location: location.clone(),
        }));

        negations
    }

    fn non_doc_comments(
        &self,
        file_id: FileId,
        root: &SyntaxNode,
        CrateModuleGraph { violations, .. }: &mut CrateModuleGraph,
    ) {
        violations.extend(
            root.descendants_with_tokens()
                .filter_map(NodeOrToken::into_token)
                .filter_map(ast::Comment::cast)
                .map(|comment| Violation {
                    kind: ViolationKind::NonDocComment,
                    location: self.locator.locate(FileRangeWrapper {
                        file_id,
                        range: comment.syntax().text_range(),
                    }),
                }),
        );
    }

    fn order_violations<TKey: Ord>(
        &self,
        OrderedEntries { entries }: OrderedEntries<TKey>,
        kind: impl Fn(&TKey, &TKey, String) -> ViolationKind,
        origin: &SubtreeOrigin,
    ) -> Vec<Violation> {
        OrderedEntries {
            entries: entries
                .into_iter()
                .filter(|OrderedEntry { node, .. }| {
                    self.token_origin.is_own_text(
                        self.semantics.hir_file_for(node),
                        ItemFocus {
                            node,
                            semantics: self.semantics,
                        }
                        .range(),
                    )
                })
                .collect(),
        }
        .misordered()
        .iter()
        .map(|MisorderedEntry { entry, preceding }| Violation {
            kind: kind(
                &entry.key,
                &preceding.key,
                ItemFocus {
                    node: &preceding.node,
                    semantics: self.semantics,
                }
                .text(),
            ),
            location: self.locate_focus(&entry.node, origin),
        })
        .collect()
    }

    fn own_violation(
        &self,
        kind: ViolationKind,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
    ) -> Option<Violation> {
        self.token_origin
            .is_own_text(self.semantics.hir_file_for(node), node.text_range())
            .then(|| Violation {
                kind,
                location: self.locate(node, origin),
            })
    }

    fn parameter_violations(
        &self,
        parameter: &ast::Param,
        origin: &SubtreeOrigin,
    ) -> Vec<Violation> {
        parameter
            .pat()
            .into_iter()
            .flat_map(|pattern| pattern.syntax().descendants())
            .filter_map(ast::IdentPat::cast)
            .filter(|binding| self.semantics.resolve_bind_pat_to_const(binding).is_none())
            .filter_map(|binding| binding.name())
            .filter_map(|name| {
                ParameterName {
                    identifier: name.as_name().as_str(),
                }
                .single_letter_violation()
                .and_then(|kind| self.own_violation(kind, name.syntax(), origin))
            })
            .chain(self.undestructured_parameter(parameter, origin))
            .collect()
    }

    fn procedural_function(
        &self,
        function: Function,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
    ) -> Option<Violation> {
        let database = self.semantics.db;

        function
            .params_without_self(database)
            .iter()
            .find_map(|parameter| {
                parameter
                    .ty()
                    .strip_references()
                    .as_adt()
                    .filter(|adt| self.is_own_crate_adt(*adt))
            })
            .filter(|_| !self.is_exempt_from_the_procedural_rule(function, node))
            .map(|adt| Violation {
                kind: ViolationKind::ProceduralFunction {
                    type_name: adt
                        .name(database)
                        .display(database, self.krate.edition(database))
                        .to_string(),
                },
                location: self.locate_focus(node, origin),
            })
    }

    fn push_expansion(
        &self,
        expansion: MacroCallId,
        call: &SyntaxNode,
        origin: &SubtreeOrigin,
        pending: &mut Vec<Subtree>,
        CrateModuleGraph { violations, .. }: &mut CrateModuleGraph,
    ) {
        if (MacroExpansion {
            call: expansion,
            database: self.semantics.db,
        })
        .syntax_errors()
        .is_empty()
        {
            pending.push(Subtree {
                origin: origin.expansion_of(call),
                root: self.semantics.parse_or_expand(HirFileId::from(expansion)),
            });
        } else {
            self.unparsable(expansion, self.locate(call, origin), violations);
        }
    }

    fn record(
        &self,
        path: &ast::Path,
        origin: &SubtreeOrigin,
        CrateModuleGraph {
            dependencies,
            unresolved_foreign_references,
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
            PathReferenceOutcome::UnresolvedInForeignModule { crate_name, module } => {
                unresolved_foreign_references.insert(UnresolvedForeignReference {
                    location: self.locate(path.syntax(), origin),
                    module: ForeignModule {
                        file: self.locator.file(original_file_id(
                            self.semantics.db,
                            module.definition_source_file_id(self.semantics.db),
                        )),
                        name: iter::once(crate_name)
                            .chain(self.module_paths.path(module).segments)
                            .collect::<Vec<String>>()
                            .join("::"),
                    },
                });
            }
        }
    }

    fn record_own_tokens(
        &self,
        file: HirFileId,
        tokens: &[SyntaxToken],
        recorded: &mut WrittenTokens,
    ) {
        for token in tokens {
            let written = self.token_origin.written_range(file, token.text_range());

            if self.token_origin.is_own_text(file, token.text_range()) {
                recorded.insert(
                    self.locator.file(written.file_id),
                    u32::from(written.range.start()),
                );
            }
        }
    }

    fn signature_violations(&self, node: &SyntaxNode, origin: &SubtreeOrigin) -> Vec<Violation> {
        ast::Param::cast(node.clone()).map_or_else(
            || {
                ast::TupleType::cast(node.clone())
                    .filter(|tuple| SignatureTuple { tuple }.is_in_a_signature())
                    .and_then(|tuple| {
                        self.own_violation(ViolationKind::TupleInSignature, tuple.syntax(), origin)
                    })
                    .or_else(|| self.generic_parameter_violation(node, origin))
                    .into_iter()
                    .collect()
            },
            |parameter| self.parameter_violations(&parameter, origin),
        )
    }

    fn test_only_code(&self, node: &SyntaxNode, origin: &SubtreeOrigin) -> Option<Violation> {
        (self.is_test_gated(node)
            && !self.token_origin.is_foreign_text(node)
            && !self.test_code.contains(node)
            && !Ancestry {
                node,
                semantics: self.semantics,
            }
            .nodes()
            .iter()
            .skip(1)
            .any(|enclosing| self.is_test_gated(enclosing)))
        .then(|| Violation {
            kind: ViolationKind::TestOnlyCodeInProduction,
            location: self.locate_focus(node, origin),
        })
    }

    fn test_target_violations(
        &self,
        items: &[ast::Item],
        origin: &SubtreeOrigin,
        CrateModuleGraph { violations, .. }: &mut CrateModuleGraph,
        TestTargetFindings {
            accepted,
            impl_blocks,
            types,
        }: &mut TestTargetFindings,
    ) {
        for item in items {
            let violation = Violation {
                kind: ViolationKind::NonTestItemInTestTarget,
                location: self.locate_focus(item.syntax(), origin),
            };

            if let ItemDefinition::ModuleDef(definition) = ItemDefinition::new(item, self.semantics)
                && (FileLocalItem {
                    definition,
                    file_id: original_file_id(
                        self.semantics.db,
                        self.semantics.hir_file_for(item.syntax()),
                    ),
                    semantics: self.semantics,
                })
                .is_file_local()
            {
                if let ModuleDef::Adt(adt) = definition {
                    accepted.push(adt);
                }
            } else if let Some(adt) = (ItemAdt {
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

    fn undestructured_parameter(
        &self,
        parameter: &ast::Param,
        origin: &SubtreeOrigin,
    ) -> Option<Violation> {
        let database = self.semantics.db;

        parameter
            .pat()
            .filter(|_| matches!(origin, SubtreeOrigin::SourceFile))
            .and_then(|pattern| match pattern {
                ast::Pat::IdentPat(binding) if binding.pat().is_none() => Some(binding),
                _ => None,
            })
            .and_then(|binding| {
                FieldOnlyParameter {
                    binding: &binding,
                    semantics: self.semantics,
                    token_origin: self.token_origin,
                }
                .destructurable_type()
                .map(|held| Violation {
                    kind: ViolationKind::UndestructuredParameter {
                        type_name: held
                            .display(database, self.krate.to_display_target(database))
                            .to_string(),
                    },
                    location: self.locate_focus(binding.syntax(), origin),
                })
            })
    }

    fn unparsable(
        &self,
        call: MacroCallId,
        location: SourceLocation,
        violations: &mut ViolationSet,
    ) {
        let violation = Violation {
            kind: ViolationKind::UnparsableMacroExpansion,
            location,
        };

        violations.insert(violation.clone());

        for finding in (ExpansionChain {
            call,
            locator: self.locator,
            semantics: self.semantics,
        })
        .findings()
        {
            violations.fold(violation.clone(), finding);
        }
    }

    fn unresolved_method_call(
        &self,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
    ) -> Option<Violation> {
        ast::MethodCallExpr::cast(node.clone())
            .filter(|call| {
                self.is_unresolved_method_call(call)
                    && !call.receiver().is_some_and(|receiver| {
                        self.contains_unresolved_reference(receiver.syntax())
                    })
            })
            .and_then(|call| call.name_ref())
            .filter(|name| !self.token_origin.is_foreign_text(name.syntax()))
            .map(|name| Violation {
                kind: ViolationKind::UnresolvedMethodCall,
                location: self.locate(name.syntax(), origin),
            })
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
        graph
            .violations
            .extend(self.unresolved_method_call(node, origin));

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

            if matches!(origin, SubtreeOrigin::SourceFile) {
                graph
                    .violations
                    .extend(self.method_order_violations(&written, origin));
            }
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
            && let ItemDefinition::ModuleDef(ModuleDef::Function(function)) = definition
        {
            graph
                .violations
                .extend(self.procedural_function(*function, node, origin));
            graph
                .violations
                .extend(self.function_used_only_by_tests(*function, node, origin));
        } else if matches!(origin, SubtreeOrigin::SourceFile)
            && Self::is_public_item_container(node)
        {
            self.container(node, origin, graph, test_target);
        } else {
            graph
                .violations
                .extend(self.signature_violations(node, origin));
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

                    let negations = self.negated_attributes(&configured, &subtree.origin, graph);

                    let predicate = self.inactive_code.get(&InFile::new(
                        self.semantics.hir_file_for(&node),
                        SyntaxNodePtr::new(&node),
                    ));

                    if configured.is_disabled() {
                        preorder.skip_subtree();
                        self.inactive(&node, negations, graph);
                    } else if let Some(expression) = predicate {
                        preorder.skip_subtree();

                        let mut predicate_negations = BTreeSet::new();

                        if (NegatedConfiguration { expression }).is_negated() {
                            let location = self.locate_focus(&node, &subtree.origin);

                            graph.configured_out_negations.negations.insert(Violation {
                                kind: ViolationKind::NegatedConfiguration,
                                location: location.clone(),
                            });
                            predicate_negations.insert(location);
                        }

                        self.inactive(&node, predicate_negations, graph);
                    } else {
                        self.activate(&node, graph);
                        self.lint_suppressions(&node, &subtree.origin, graph);
                        graph
                            .violations
                            .extend(self.test_only_code(&node, &subtree.origin));

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
                            ItemExpansion::Unparsable { call } => {
                                preorder.skip_subtree();
                                self.unparsable(
                                    call,
                                    self.locate_focus(&node, &subtree.origin),
                                    &mut graph.violations,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}
