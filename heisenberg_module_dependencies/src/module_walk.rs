use std::borrow::Cow;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::iter;

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
use ra_ap_hir::ModuleDef;
use ra_ap_hir::ModuleId;
use ra_ap_hir::Semantics;
use ra_ap_hir::Type;
use ra_ap_hir_def::visibility::visibility_from_ast;
use ra_ap_hir_expand::Lookup;
use ra_ap_hir_expand::MacroDefKind;
use ra_ap_hir_expand::builtin::BuiltinFnLikeExpander;
use ra_ap_ide_db::RootDatabase;
use ra_ap_paths::AbsPath;
use ra_ap_stdx::to_lower_snake_case;
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
use ra_ap_syntax::ast::HasGenericArgs as _;
use ra_ap_syntax::ast::HasName as _;
use ra_ap_syntax::ast::HasVisibility as _;
use ra_ap_vfs::FileId;

use heisenberg_module_graph::configured_out_negations::ConfiguredOutNegations;
use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_module_graph::crate_target::CrateTarget;
use heisenberg_module_graph::crate_target_kind::CrateTargetKind;
use heisenberg_module_graph::foreign_module::ForeignModule;
use heisenberg_module_graph::inactive_focus::InactiveFocus;
use heisenberg_module_graph::module_dependency::ModuleDependency;
use heisenberg_module_graph::module_file_analysis::ModuleFileAnalysis;
use heisenberg_module_graph::shared_module_file::SharedModuleFile;
use heisenberg_module_graph::source_location::SourceLocation;
use heisenberg_module_graph::unresolved_foreign_reference::UnresolvedForeignReference;
use heisenberg_module_graph::violation::Violation;
use heisenberg_module_graph::violation_kind::ViolationKind;
use heisenberg_module_graph::walked_code::WalkedCode;
use heisenberg_module_graph::written_tokens::WrittenTokens;

use crate::adt_violation::AdtViolation;
use crate::code_tokens::code_tokens;
use crate::configured_node::ConfiguredNode;
use crate::constants_module::ConstantsModule;
use crate::file_local_item::FileLocalItem;
use crate::impl_home::ImplHome;
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
use crate::original_file_id::original_file_id;
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
    pub cargo_variables: &'walk BTreeSet<String>,
    pub crate_target: &'walk CrateTarget,
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

    fn environment_variable_violation(
        &self,
        call: &ast::MacroCall,
        origin: &SubtreeOrigin,
    ) -> Option<Violation> {
        let location = self.locate_focus(call.syntax(), origin);
        let variable = call
            .token_tree()
            .and_then(|tree| {
                tree.syntax()
                    .children_with_tokens()
                    .filter_map(NodeOrToken::into_token)
                    .filter(|token| !token.kind().is_trivia())
                    .nth(1)
            })
            .and_then(ast::String::cast)
            .and_then(|literal| literal.value().ok().map(Cow::into_owned));

        match variable {
            None => Some(Violation {
                kind: ViolationKind::ComputedEnvironmentVariableName,
                location,
            }),
            Some(variable)
                if self.cargo_variables.contains(&variable)
                    || self
                        .krate
                        .base()
                        .env(self.semantics.db)
                        .get(&variable)
                        .is_some() =>
            {
                None
            }
            Some(variable) => Some(Violation {
                kind: ViolationKind::UndeclaredEnvironmentVariable { variable },
                location,
            }),
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

            if let MacroDefKind::BuiltInEager(_, expander) = call_location.def.kind
                && expander.is_env_or_option_env()
                && !self.token_origin.is_foreign_text(call.syntax())
            {
                graph
                    .violations
                    .extend(self.environment_variable_violation(call, origin));
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

        if self.token_origin.is_own_file(focus.file_id) {
            inactive_foci.insert(InactiveFocus {
                location: self.locator.locate(focus),
                negations,
                token_starts: tokens
                    .iter()
                    .filter(|token| focus_range.contains_range(token.text_range()))
                    .map(|token| self.token_origin.written_range(file, token.text_range()))
                    .filter(|written| written.file_id == focus.file_id)
                    .map(|written| u32::from(written.range.start()))
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
        let written_in = original_file_id(
            database,
            self.semantics.hir_file_for(implementation.syntax()),
        );
        let location = self.locate_focus(implementation.syntax(), origin);
        let trait_arguments: Vec<Type<'_>> = implementation
            .trait_()
            .and_then(|written_trait| ast::PathType::cast(written_trait.syntax().clone()))
            .and_then(|path_type| path_type.path())
            .and_then(|path| path.segment())
            .and_then(|segment| segment.generic_arg_list())
            .into_iter()
            .flat_map(|arguments| arguments.generic_args())
            .filter_map(|argument| ast::TypeArg::cast(argument.syntax().clone()))
            .filter_map(|argument| argument.ty())
            .filter_map(|written_type| self.semantics.resolve_type(&written_type))
            .collect();

        match (ImplementedType {
            database,
            implementation: definition,
            krate: self.krate,
        })
        .home(&trait_arguments)
        {
            ImplHome::LocalType { file }
            | ImplHome::LocalTraitBlanket { file }
            | ImplHome::LocalTraitForeignType { file, .. }
            | ImplHome::LocalTraitObject { file }
            | ImplHome::LocalTypeArgument { file }
                if file == written_in =>
            {
                None
            }
            ImplHome::LocalType { file } | ImplHome::LocalTypeArgument { file } => {
                Some(Violation {
                    kind: ViolationKind::MisplacedImpl {
                        type_file: self.locator.file(file),
                    },
                    location,
                })
            }
            ImplHome::LocalTraitBlanket { file } => Some(Violation {
                kind: ViolationKind::MisplacedBlanketImpl {
                    trait_file: self.locator.file(file),
                },
                location,
            }),
            ImplHome::LocalTraitObject { file } => Some(Violation {
                kind: ViolationKind::MisplacedTraitObjectImpl {
                    trait_file: self.locator.file(file),
                },
                location,
            }),
            ImplHome::LocalTraitForeignType { file, trait_name } => Some(Violation {
                kind: ViolationKind::MisplacedForeignTypeImpl {
                    trait_file: self.locator.file(file),
                    trait_name,
                },
                location,
            }),
            ImplHome::Foreign => Some(Violation {
                kind: ViolationKind::ImplWithoutLocalType,
                location,
            }),
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
                        .map(|name| to_lower_snake_case(name.text()))
                        .filter(|expected| *expected != module)
                        .map(|expected| Violation {
                            kind: ViolationKind::ModuleNotNamedAfterItsItem { expected, module },
                            location: self.locate_focus(item.syntax(), origin),
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

            if self.token_origin.is_own_file(written.file_id) {
                recorded.insert(
                    self.locator.file(written.file_id),
                    u32::from(written.range.start()),
                );
            }
        }
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
