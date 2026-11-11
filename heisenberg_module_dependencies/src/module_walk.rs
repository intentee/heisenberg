use std::cell::OnceCell;
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
use ra_ap_hir::PathResolution;
use ra_ap_hir::PathResolutionPerNs;
use ra_ap_hir::Semantics;
use ra_ap_hir::Trait;
use ra_ap_hir_def::FunctionId;
use ra_ap_hir_def::MacroId;
use ra_ap_hir_def::rustc_abi::ExternAbi;
use ra_ap_hir_def::signatures::FunctionSignature;
use ra_ap_hir_def::visibility::visibility_from_ast;
use ra_ap_hir_expand::Lookup;
use ra_ap_hir_expand::MacroDefKind;
use ra_ap_hir_expand::builtin::BuiltinFnLikeExpander;
use ra_ap_hir_expand::builtin::EagerExpander;
use ra_ap_hir_expand::name::AsName as _;
use ra_ap_hir_expand::name::Name;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::defs::Definition;
use ra_ap_ide_db::search::FileReference;
use ra_ap_paths::AbsPath;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::AstToken;
use ra_ap_syntax::NodeOrToken;
use ra_ap_syntax::Preorder;
use ra_ap_syntax::RustLanguage;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::SyntaxNodePtr;
use ra_ap_syntax::SyntaxToken;
use ra_ap_syntax::T;
use ra_ap_syntax::TextRange;
use ra_ap_syntax::TextSize;
use ra_ap_syntax::WalkEvent;
use ra_ap_syntax::ast;
use ra_ap_syntax::ast::HasName as _;
use ra_ap_syntax::ast::HasVisibility as _;
use ra_ap_vfs::FileId;

use heisenberg_configured_code::ancestry::Ancestry;
use heisenberg_configured_code::configured_node::ConfiguredNode;
use heisenberg_configured_code::negated_configuration::NegatedConfiguration;
use heisenberg_configured_code::test_code::TestCode;
use heisenberg_expanded_code::code_tokens::code_tokens;
use heisenberg_module_graph::configured_out_negations::ConfiguredOutNegations;
use heisenberg_module_graph::crate_target::CrateTarget;
use heisenberg_module_graph::crate_target_kind::CrateTargetKind;
use heisenberg_module_graph::foreign_module::ForeignModule;
use heisenberg_module_graph::inactive_focus::InactiveFocus;
use heisenberg_module_graph::item_group::ItemGroup;
use heisenberg_module_graph::item_placement::ItemPlacement;
use heisenberg_module_graph::method_placement::MethodPlacement;
use heisenberg_module_graph::module_dependency::ModuleDependency;
use heisenberg_module_graph::module_file_analysis::ModuleFileAnalysis;
use heisenberg_module_graph::module_findings::ModuleFindings;
use heisenberg_module_graph::shared_module_file::SharedModuleFile;
use heisenberg_module_graph::source_location::SourceLocation;
use heisenberg_module_graph::unresolved_foreign_reference::UnresolvedForeignReference;
use heisenberg_module_graph::violation::Violation;
use heisenberg_module_graph::violation_kind::ViolationKind;
use heisenberg_module_graph::violation_set::ViolationSet;
use heisenberg_module_graph::walked_code::WalkedCode;
use heisenberg_module_graph::written_tokens::WrittenTokens;
use heisenberg_project_model::source_file::SourceFile;
use heisenberg_source_locator::original_file_id::original_file_id;
use heisenberg_source_locator::source_locator::SourceLocator;
use heisenberg_source_locator::written_range::written_range;
use heisenberg_task_graph::once_map::OnceMap;

use crate::adt_violation::AdtViolation;
use crate::attributed_item::AttributedItem;
use crate::constants::LIFETIME_PREFIX;
use crate::constants_module::ConstantsModule;
use crate::constructor::Constructor;
use crate::covered_code::CoveredCode;
use crate::crate_findings::CrateFindings;
use crate::defined_items::DefinedItems;
use crate::definition_usages::DefinitionUsages;
use crate::expanded_items::ExpandedItems;
use crate::expansion_chain::ExpansionChain;
use crate::field_only_parameter::FieldOnlyParameter;
use crate::file_local_item::FileLocalItem;
use crate::function_bodies::FunctionBodies;
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
use crate::local_macro::LocalMacro;
use crate::local_macro_import::LocalMacroImport;
use crate::macro_expansion::MacroExpansion;
use crate::method_grouping::MethodGrouping;
use crate::method_key::MethodKey;
use crate::method_order::MethodOrder;
use crate::method_owner::MethodOwner;
use crate::misordered_entry::MisorderedEntry;
use crate::module_dependencies_error::ModuleDependenciesError;
use crate::module_item::ModuleItem;
use crate::module_paths::ModulePaths;
use crate::name_words::NameWords;
use crate::ordered_entries::OrderedEntries;
use crate::ordered_entry::OrderedEntry;
use crate::outside_visibility::OutsideVisibility;
use crate::parameter_name::ParameterName;
use crate::path_reference::PathReference;
use crate::path_reference_outcome::PathReferenceOutcome;
use crate::path_resolutions::PathResolutions;
use crate::public_items::PublicItems;
use crate::signature_tuple::SignatureTuple;
use crate::subtree::Subtree;
use crate::subtree_origin::SubtreeOrigin;
use crate::template_captures::TemplateCaptures;
use crate::test_target_findings::TestTargetFindings;
use crate::test_target_items::TestTargetItems;
use crate::token_origin::TokenOrigin;
use crate::trait_arguments::trait_arguments;
use crate::unsafe_code_findings::UnsafeCodeFindings;
use crate::walked_module::WalkedModule;
use crate::written_token::WrittenToken;

pub struct ModuleWalk<'walk, 'database> {
    pub build_script_output_files: &'walk BTreeSet<FileId>,
    pub crate_target: &'walk CrateTarget,
    pub file_modules: &'walk HashMap<FileId, Vec<Module>>,
    pub inactive_code: &'walk HashMap<InFile<SyntaxNodePtr>, CfgExpr>,
    pub krate: Crate,
    pub locator: &'walk SourceLocator<'walk>,
    pub module_paths: &'walk ModulePaths,
    pub options: &'walk CfgOptions,
    pub paths: &'walk PathResolutions<'walk, 'database>,
    pub production_options: &'walk CfgOptions,
    pub semantics: &'walk Semantics<'database, RootDatabase>,
    pub template_captures: &'walk TemplateCaptures<'walk, 'database>,
    pub test_code: &'walk TestCode<'walk, 'database>,
    pub test_gated_code: &'walk OnceMap<InFile<SyntaxNodePtr>, bool>,
    pub token_origin: &'walk TokenOrigin<'walk, 'database>,
    pub unpublished_packages: &'walk BTreeSet<String>,
}

impl ModuleWalk<'_, '_> {
    pub fn module(
        &self,
        module: Module,
        walked: &mut WalkedModule,
    ) -> Result<(), ModuleDependenciesError> {
        let database = self.semantics.db;

        module
            .as_source_file_id(database)
            .map_or(Ok(()), |source_file_id| {
                let file_id = source_file_id.file_id(database);
                let file_modules = &self.file_modules[&file_id];

                self.file_start(file_id)
                    .and_then(|SourceLocation { file, .. }| {
                        let analysis = file_modules
                            .first()
                            .filter(|first| **first != module)
                            .map_or(ModuleFileAnalysis::Analysed, |first| {
                                ModuleFileAnalysis::AlreadyAnalysedAs(
                                    self.module_paths.path(database, *first),
                                )
                            });

                        if file_modules.len() > 1 {
                            walked
                                .findings
                                .shared_module_files
                                .insert(SharedModuleFile {
                                    analysis: analysis.clone(),
                                    file: file.clone(),
                                    module: self.module_paths.path(database, module),
                                });
                        }

                        if analysis == ModuleFileAnalysis::Analysed {
                            let root = self.semantics.parse(source_file_id).syntax().clone();

                            walked.findings.walked_code.files.insert(file);
                            self.non_doc_comments(file_id, &root, &mut walked.findings)
                                .and_then(|()| {
                                    self.walk(
                                        Subtree {
                                            origin: SubtreeOrigin::SourceFile,
                                            root: root.clone(),
                                        },
                                        walked,
                                    )
                                })
                                .and_then(|()| {
                                    FunctionBodies {
                                        crate_target: self.crate_target,
                                        locator: self.locator,
                                        semantics: self.semantics,
                                        template_captures: self.template_captures,
                                    }
                                    .written_in(&root)
                                    .map(|bodies| walked.function_bodies.extend(bodies))
                                })
                        } else {
                            Ok(())
                        }
                    })
            })
    }

    fn activate(
        &self,
        node: &SyntaxNode,
        file: HirFileId,
        ModuleFindings {
            walked_code: WalkedCode { active_tokens, .. },
            ..
        }: &mut ModuleFindings,
    ) -> Result<(), ModuleDependenciesError> {
        if ast::TokenTree::can_cast(node.kind()) {
            Ok(())
        } else {
            let tokens = self.written_tokens(file, code_tokens(node.children_with_tokens()));

            self.record_own_tokens(file, &tokens, active_tokens)
        }
    }

    fn configured_out(
        &self,
        node: &SyntaxNode,
        file: HirFileId,
        expression: &CfgExpr,
        origin: &SubtreeOrigin,
        module_findings: &mut ModuleFindings,
    ) -> Result<(), ModuleDependenciesError> {
        if (NegatedConfiguration { expression }).is_negated() {
            self.locate_focus(node, origin).and_then(|location| {
                module_findings
                    .configured_out_negations
                    .negations
                    .insert(Violation {
                        kind: ViolationKind::NegatedConfiguration,
                        location: location.clone(),
                    });
                self.inactive(node, file, BTreeSet::from([location]), module_findings)
            })
        } else {
            self.inactive(node, file, BTreeSet::new(), module_findings)
        }
    }

    fn container(
        &self,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
        module_findings: &mut ModuleFindings,
        CrateFindings { test_target, .. }: &mut CrateFindings,
    ) -> Result<(), ModuleDependenciesError> {
        let defined = DefinedItems {
            container: node,
            options: self.options,
            semantics: self.semantics,
        }
        .items();
        let expanded = ExpandedItems {
            options: self.options,
            semantics: self.semantics,
            token_origin: self.token_origin,
        }
        .of(defined.clone());
        let non_constant = PublicItems {
            items: &expanded,
            options: self.options,
            semantics: self.semantics,
            token_origin: self.token_origin,
        }
        .non_constant();

        self.item_order_violations(&defined, origin)
            .and_then(|misordered| {
                module_findings.violations.extend(misordered);

                if self.is_constants_module(node) {
                    self.item_violations(
                        non_constant.iter(),
                        &ViolationKind::NonConstantInConstantsModule,
                        origin,
                    )
                    .map(|non_constants| module_findings.violations.extend(non_constants))
                } else {
                    self.module_item(node, &non_constant)
                        .and_then(|module_item| {
                            let accepted = module_item.item();

                            self.item_violations(
                                non_constant.iter().filter(|item| Some(*item) != accepted),
                                &ViolationKind::AdditionalPublicItem,
                                origin,
                            )
                            .and_then(|additional| {
                                module_findings.violations.extend(additional);
                                self.module_naming_violation(module_item, origin)
                            })
                            .map(|misnamed| module_findings.violations.extend(misnamed))
                        })
                }
            })
            .and_then(|()| {
                if self.crate_target.kind == CrateTargetKind::IntegrationTest {
                    self.test_target_violations(
                        &TestTargetItems {
                            crate_target: self.crate_target,
                            items: &expanded,
                            paths: self.paths,
                            semantics: self.semantics,
                        }
                        .neither_tests_constants_nor_imports(),
                        origin,
                        module_findings,
                        test_target,
                    )
                } else {
                    Ok(())
                }
            })
    }

    fn contains_unresolved_reference(&self, node: &SyntaxNode) -> bool {
        node.descendants().any(|descendant| {
            ast::Path::cast(descendant.clone())
                .is_some_and(|path| self.paths.resolve(&path).is_none())
                || ast::MethodCallExpr::cast(descendant)
                    .is_some_and(|call| self.is_unresolved_method_call(&call))
        })
    }

    fn declare(
        &self,
        declaration: &ast::Module,
        module: Module,
        origin: &SubtreeOrigin,
        ModuleFindings {
            configured_out_negations: ConfiguredOutNegations { declarations, .. },
            violations,
            ..
        }: &mut ModuleFindings,
    ) -> Result<(), ModuleDependenciesError> {
        let database = self.semantics.db;

        if is_module_inside_block(database, module) {
            self.locate_focus(declaration.syntax(), origin)
                .map(|location| {
                    violations.insert(Violation {
                        kind: ViolationKind::ModuleFileInsideBlock,
                        location,
                    });
                })
        } else if let Some(source_file_id) = module.as_source_file_id(database)
            && !(ConfiguredNode {
                node: self.semantics.parse(source_file_id).syntax(),
                options: self.options,
            })
            .negated_attributes()
            .is_empty()
        {
            self.locate_focus(declaration.syntax(), origin)
                .map(|location| {
                    declarations.insert(location);
                })
        } else {
            Ok(())
        }
    }

    fn declared_method_order_violations(
        &self,
        items: &ast::AssocItemList,
        implemented: Trait,
        origin: &SubtreeOrigin,
    ) -> Result<Vec<Violation>, ModuleDependenciesError> {
        let database = self.semantics.db;
        let declared = implemented.items(database);

        self.order_violations(
            MethodOrder {
                items,
                options: self.options,
                semantics: self.semantics,
            }
            .entries(|method| {
                let name = method.name(database);

                declared.iter().position(|item| {
                    item.as_function()
                        .is_some_and(|declared_function| declared_function.name(database) == name)
                })
            }),
            |_: &usize, _: &usize, preceding| ViolationKind::MethodOutOfOrder {
                placement: MethodPlacement::Declared {
                    preceding,
                    trait_name: implemented.name(database).as_str().to_owned(),
                },
            },
            origin,
        )
    }

    fn enter(
        &self,
        node: &SyntaxNode,
        file: HirFileId,
        origin: &SubtreeOrigin,
        preorder: &mut Preorder<RustLanguage>,
        pending: &mut Vec<Subtree>,
        walked: &mut WalkedModule,
    ) -> Result<(), ModuleDependenciesError> {
        let configured = ConfiguredNode {
            node,
            options: self.options,
        };

        self.negated_attributes(&configured, origin, &mut walked.findings)
            .and_then(|negations| {
                let predicate = self
                    .inactive_code
                    .get(&InFile::new(file, SyntaxNodePtr::new(node)));

                if configured.is_disabled() {
                    preorder.skip_subtree();
                    self.inactive(node, file, negations, &mut walked.findings)
                } else if let Some(expression) = predicate {
                    preorder.skip_subtree();
                    self.configured_out(node, file, expression, origin, &mut walked.findings)
                } else {
                    self.activate(node, file, &mut walked.findings)
                        .and_then(|()| {
                            self.unsafe_code(
                                node,
                                file,
                                origin,
                                &mut walked.crate_findings.unsafe_code,
                            )
                        })
                        .and_then(|()| {
                            self.unresolved_captures(node, file, origin, &mut walked.findings)
                        })
                        .and_then(|()| self.lint_suppressions(node, origin, &mut walked.findings))
                        .and_then(|()| self.test_only_code(node, origin))
                        .and_then(|test_only| {
                            walked.findings.violations.extend(test_only);
                            self.item(node, origin, preorder, pending, walked)
                        })
                }
            })
    }

    fn expand(
        &self,
        call: &ast::MacroCall,
        origin: &SubtreeOrigin,
        pending: &mut Vec<Subtree>,
        module_findings: &mut ModuleFindings,
    ) -> Result<(), ModuleDependenciesError> {
        let database = self.semantics.db;

        self.semantics.to_def(call).map_or(Ok(()), |expansion| {
            let call_location = expansion.lookup(database);

            call_location
                .include_file_id(database, expansion)
                .map(|included| included.file_id(database))
                .filter(|included| {
                    !self.token_origin.is_foreign_text(call.syntax())
                        && !self.build_script_output_files.contains(included)
                })
                .map_or(Ok(()), |included| {
                    self.locator.file(included).and_then(|file| {
                        self.locate_focus(call.syntax(), origin).map(|location| {
                            module_findings.violations.insert(Violation {
                                kind: ViolationKind::IncludedSourceFile { file },
                                location,
                            });
                        })
                    })
                })
                .and_then(|()| {
                    if matches!(
                        call_location.def.kind,
                        MacroDefKind::BuiltIn(_, BuiltinFnLikeExpander::CfgSelect)
                    ) && !self.token_origin.is_foreign_text(call.syntax())
                    {
                        self.locate_focus(call.syntax(), origin).map(|location| {
                            module_findings.violations.insert(Violation {
                                kind: ViolationKind::ConfigurationSelectionMacro,
                                location,
                            });
                        })
                    } else {
                        Ok(())
                    }
                })
                .and_then(|()| {
                    pending.extend(expansion.eager_arg(database).map(|argument| Subtree {
                        origin: origin.expansion_of(call.syntax()),
                        root: self.semantics.parse_or_expand(HirFileId::from(argument)),
                    }));

                    self.push_expansion(expansion, call.syntax(), origin, pending, module_findings)
                })
        })
    }

    fn expand_derives(
        &self,
        derive: &ast::Meta,
        origin: &SubtreeOrigin,
        pending: &mut Vec<Subtree>,
        module_findings: &mut ModuleFindings,
    ) -> Result<(), ModuleDependenciesError> {
        self.semantics
            .expand_derive_macro(derive)
            .into_iter()
            .flatten()
            .flatten()
            .filter_map(|ExpandResult { value, .. }| {
                self.semantics.hir_file_for(&value).macro_file()
            })
            .try_for_each(|expansion| {
                self.push_expansion(expansion, derive.syntax(), origin, pending, module_findings)
            })
    }

    fn file_start(&self, file_id: FileId) -> Result<SourceLocation, ModuleDependenciesError> {
        self.locator.locate(FileRangeWrapper {
            file_id,
            range: TextRange::empty(TextSize::from(0)),
        })
    }

    fn function(
        &self,
        function: Function,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
        ModuleFindings { violations, .. }: &mut ModuleFindings,
    ) -> Result<(), ModuleDependenciesError> {
        let usages = DefinitionUsages::new(ModuleDef::Function(function), self.semantics);

        self.procedural_function(function, node, origin, &usages)
            .and_then(|procedural| {
                violations.extend(procedural);
                self.function_used_only_by_tests(function, node, origin, &usages)
            })
            .and_then(|used_only_by_tests| {
                violations.extend(used_only_by_tests);
                self.referenced_test(node, origin, &usages)
            })
            .map(|referenced| violations.extend(referenced))
    }

    fn function_used_only_by_tests(
        &self,
        function: Function,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
        usages: &DefinitionUsages<'_, '_>,
    ) -> Result<Option<Violation>, ModuleDependenciesError> {
        let database = self.semantics.db;

        (function
            .as_assoc_item(database)
            .is_none_or(|item| item.container_or_implemented_trait(database).is_none())
            && (self
                .unpublished_packages
                .contains(&self.crate_target.package_name)
                || !self.is_reachable_from_other_crates(function))
            && !self.test_code.contains(node)
            && !self.in_test_gated_code(node)
            && {
                let references = &usages.all().references;

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
        .then(|| {
            self.locate_focus(node, origin).map(|location| Violation {
                kind: ViolationKind::FunctionUsedOnlyByTests,
                location,
            })
        })
        .transpose()
    }

    fn generic_parameter_violation(
        &self,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
    ) -> Result<Option<Violation>, ModuleDependenciesError> {
        ast::LifetimeParam::cast(node.clone())
            .and_then(|parameter| parameter.lifetime())
            .and_then(|lifetime| {
                Name::new_lifetime(lifetime.text())
                    .as_str()
                    .strip_prefix(LIFETIME_PREFIX)
                    .and_then(|identifier| ParameterName { identifier }.single_letter_violation())
                    .map(|kind| self.own_violation(kind, lifetime.syntax(), origin))
            })
            .or_else(|| {
                ast::TypeParam::cast(node.clone())
                    .and_then(|parameter| parameter.name())
                    .and_then(|name| {
                        ParameterName {
                            identifier: name.as_name().as_str(),
                        }
                        .type_parameter_violation()
                        .map(|kind| self.own_violation(kind, name.syntax(), origin))
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
                        .map(|kind| self.own_violation(kind, name.syntax(), origin))
                    })
            })
            .transpose()
            .map(Option::flatten)
    }

    fn grouped_method_order_violations(
        &self,
        items: &ast::AssocItemList,
        owner: MethodOwner,
        origin: &SubtreeOrigin,
    ) -> Result<Vec<Violation>, ModuleDependenciesError> {
        let grouping = MethodGrouping {
            krate: self.krate,
            owner,
            semantics: self.semantics,
        };

        self.order_violations(
            MethodOrder {
                items,
                options: self.options,
                semantics: self.semantics,
            }
            .entries(|method| Some(grouping.key(method))),
            |MethodKey { group, .. }: &MethodKey,
             MethodKey {
                 group: preceding_group,
                 ..
             }: &MethodKey,
             preceding| ViolationKind::MethodOutOfOrder {
                placement: MethodPlacement::Grouped {
                    group: *group,
                    preceding,
                    preceding_group: *preceding_group,
                },
            },
            origin,
        )
    }

    fn has_own_visibility(&self, visibility: Option<ast::Visibility>) -> bool {
        visibility.is_some_and(|visibility| !self.token_origin.is_foreign_text(visibility.syntax()))
    }

    fn implementation(
        &self,
        written: &ast::Impl,
        implementation: Impl,
        origin: &SubtreeOrigin,
        ModuleFindings { violations, .. }: &mut ModuleFindings,
    ) -> Result<(), ModuleDependenciesError> {
        self.misplacement(written, implementation, origin)
            .and_then(|misplaced| {
                violations.extend(misplaced);

                if matches!(origin, SubtreeOrigin::SourceFile)
                    && let Some(items) = written.assoc_item_list()
                {
                    implementation
                        .trait_(self.semantics.db)
                        .map_or_else(
                            || {
                                self.grouped_method_order_violations(
                                    &items,
                                    MethodOwner::InherentImpl(implementation),
                                    origin,
                                )
                            },
                            |implemented| {
                                self.declared_method_order_violations(&items, implemented, origin)
                            },
                        )
                        .map(|misordered| violations.extend(misordered))
                } else {
                    Ok(())
                }
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

    fn imports_only_paths_of_local_macros(&self, import: &ast::Use) -> bool {
        let imported: Vec<ast::Path> = import
            .syntax()
            .descendants()
            .filter_map(ast::UseTree::cast)
            .filter(|tree| tree.use_tree_list().is_none())
            .filter_map(|tree| tree.path())
            .collect();

        !imported.is_empty()
            && imported.iter().all(|path| {
                LocalMacroImport {
                    path,
                    paths: self.paths,
                    semantics: self.semantics,
                }
                .imported_macro()
                    == Some(LocalMacro::CrateLocal)
            })
    }

    fn in_test_gated_code(&self, node: &SyntaxNode) -> bool {
        self.test_gated_code.get_or_init(
            InFile::new(self.semantics.hir_file_for(node), SyntaxNodePtr::new(node)),
            || {
                self.is_test_gated(node)
                    || Ancestry {
                        node,
                        semantics: self.semantics,
                    }
                    .enclosing()
                    .is_some_and(|enclosing| self.in_test_gated_code(&enclosing))
            },
        )
    }

    fn inactive(
        &self,
        node: &SyntaxNode,
        file: HirFileId,
        negations: BTreeSet<SourceLocation>,
        ModuleFindings {
            walked_code:
                WalkedCode {
                    inactive_foci,
                    inactive_tokens,
                    ..
                },
            ..
        }: &mut ModuleFindings,
    ) -> Result<(), ModuleDependenciesError> {
        let focus_range = ItemFocus {
            node,
            semantics: self.semantics,
        }
        .range();
        let focus = written_range(self.semantics.db, file, focus_range);
        let tokens = self.written_tokens(file, code_tokens(node.descendants_with_tokens()));

        self.record_own_tokens(file, &tokens, inactive_tokens)
            .and_then(|()| {
                if self
                    .token_origin
                    .is_own_written_text(file, focus_range, &focus)
                    && !self.is_compile_error_call(node)
                {
                    self.locator.locate(focus).map(|location| {
                        inactive_foci.insert(InactiveFocus {
                            location,
                            negations,
                            token_starts: tokens
                                .iter()
                                .filter(|WrittenToken { token, .. }| {
                                    focus_range.contains_range(token.text_range())
                                })
                                .filter(|WrittenToken { written, .. }| {
                                    written.file_id == focus.file_id
                                })
                                .map(|WrittenToken { written, .. }| {
                                    u32::from(written.range.start())
                                })
                                .collect(),
                        });
                    })
                } else {
                    Ok(())
                }
            })
    }

    fn integrates_foreign_code(&self, node: &SyntaxNode, file: HirFileId) -> bool {
        let database = self.semantics.db;

        (ast::ExternBlock::can_cast(node.kind())
            && self.token_origin.is_own_text(
                file,
                ItemFocus {
                    node,
                    semantics: self.semantics,
                }
                .range(),
            ))
            || ast::Fn::cast(node.clone())
                .filter(|function| {
                    function.abi().is_some_and(|abi| {
                        self.token_origin
                            .is_own_text(file, abi.syntax().text_range())
                    })
                })
                .and_then(|function| self.semantics.to_def(&function))
                .is_some_and(|function| {
                    FunctionId::try_from(function).is_ok_and(|function| {
                        FunctionSignature::of(database, function).abi != ExternAbi::Rust
                    })
                })
            || ast::Path::cast(node.clone())
                .filter(|path| {
                    self.token_origin
                        .is_own_text(file, path.syntax().text_range())
                })
                .and_then(|path| self.paths.resolve(&path))
                .is_some_and(|resolution| match resolution {
                    PathResolution::Def(ModuleDef::Function(function)) => {
                        function.extern_block(database).is_some()
                    }
                    PathResolution::Def(ModuleDef::Static(foreign)) => {
                        foreign.extern_block(database).is_some()
                    }
                    _ => false,
                })
    }

    fn is_compile_error_call(&self, node: &SyntaxNode) -> bool {
        ast::MacroCall::cast(node.clone())
            .and_then(|call| call.path())
            .and_then(|path| self.semantics.resolve_path_per_ns(&path))
            .and_then(|PathResolutionPerNs { macro_ns, .. }| macro_ns)
            .is_some_and(|resolution| {
                matches!(
                    resolution,
                    PathResolution::Def(ModuleDef::Macro(definition))
                        if matches!(
                            MacroId::from(definition).definition(self.semantics.db).kind,
                            MacroDefKind::BuiltInEager(_, EagerExpander::CompileError)
                        )
                )
            })
    }

    fn is_constants_module(&self, container: &SyntaxNode) -> bool {
        self.semantics.scope(container).is_some_and(|scope| {
            ConstantsModule {
                database: self.semantics.db,
            }
            .matches(scope.module())
        })
    }

    fn is_exempt_from_the_procedural_rule(
        &self,
        function: Function,
        node: &SyntaxNode,
        usages: &DefinitionUsages<'_, '_>,
    ) -> bool {
        let database = self.semantics.db;

        function.as_assoc_item(database).is_some_and(|item| {
            item.container_or_implemented_trait(database).is_some()
                || item.implementing_ty(database).is_some_and(|self_type| {
                    Constructor {
                        function,
                        krate: self.krate,
                        semantics: self.semantics,
                    }
                    .constructs(&self_type)
                })
        }) || self.reads_its_receiver(function)
            || FileLocalItem {
                file_id: original_file_id(database, self.semantics.hir_file_for(node)),
                usages,
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
            && function
                .as_assoc_item(database)
                .and_then(|item| item.implementing_ty(database))
                .and_then(|self_type| self_type.as_adt())
                .is_none_or(|adt| adt.visibility(database).is_public())
    }

    fn is_reexport(&self, node: &SyntaxNode, definition: &ItemDefinition) -> bool {
        let database = self.semantics.db;
        let outside = OutsideVisibility { database };

        ast::Use::cast(node.clone()).is_some_and(|import| {
            self.has_own_visibility(import.visibility())
                && !self.imports_only_paths_of_local_macros(&import)
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

    fn item(
        &self,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
        preorder: &mut Preorder<RustLanguage>,
        pending: &mut Vec<Subtree>,
        walked: &mut WalkedModule,
    ) -> Result<(), ModuleDependenciesError> {
        match ast::Item::cast(node.clone()).map_or(ItemExpansion::Unexpanded, |item| {
            ItemExpansion::new(&item, self.semantics)
        }) {
            ItemExpansion::Parsable { generated, written } => {
                preorder.skip_subtree();
                pending.extend(written.iter().map(|item| Subtree {
                    origin: origin.clone(),
                    root: item.syntax().clone(),
                }));
                pending.extend(generated.iter().map(|item| Subtree {
                    origin: origin.expansion_of(node),
                    root: item.syntax().clone(),
                }));

                Ok(())
            }
            ItemExpansion::Unexpanded => match ast::Item::cast(node.clone())
                .map_or(ItemDefinition::NotDefining, |item| {
                    ItemDefinition::new(&item, self.semantics)
                }) {
                ItemDefinition::Unmapped => {
                    preorder.skip_subtree();
                    self.locate_focus(node, origin).map(|location| {
                        walked.findings.violations.insert(Violation {
                            kind: ViolationKind::UnmappedItem,
                            location,
                        });
                    })
                }
                definition => self.visit(node, &definition, origin, pending, walked),
            },
            ItemExpansion::Unparsable { call } => {
                preorder.skip_subtree();
                self.locate_focus(node, origin).and_then(|location| {
                    self.unparsable(call, location, &mut walked.findings.violations)
                })
            }
        }
    }

    fn item_name(item: &ast::Item) -> Option<ast::Name> {
        ast::AnyHasName::cast(item.syntax().clone()).and_then(|named| named.name())
    }

    fn item_order_violations(
        &self,
        items: &[ast::Item],
        origin: &SubtreeOrigin,
    ) -> Result<Vec<Violation>, ModuleDependenciesError> {
        self.order_violations(
            ItemOrder {
                items,
                krate: self.krate,
                options: self.options,
                paths: self.paths,
                semantics: self.semantics,
                token_origin: self.token_origin,
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
    ) -> Result<Vec<Violation>, ModuleDependenciesError> {
        items
            .map(|item| {
                self.locate_focus(item.syntax(), origin)
                    .map(|location| Violation {
                        kind: kind.clone(),
                        location,
                    })
            })
            .collect()
    }

    fn lint_suppressions(
        &self,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
        ModuleFindings { violations, .. }: &mut ModuleFindings,
    ) -> Result<(), ModuleDependenciesError> {
        let holds_workspace_text = OnceCell::new();

        LintSuppressions {
            node,
            options: self.options,
            paths: self.paths,
            semantics: self.semantics,
        }
        .attributes()
        .iter()
        .filter(|meta| {
            (self.token_origin.is_workspace_text(meta.syntax())
                || self
                    .token_origin
                    .is_generated_by_a_workspace_macro(meta.syntax()))
                && *holds_workspace_text.get_or_init(|| {
                    CoveredCode {
                        options: self.options,
                        semantics: self.semantics,
                        token_origin: self.token_origin,
                    }
                    .holds_workspace_text(node)
                })
        })
        .map(|meta| {
            self.locate(meta.syntax(), origin)
                .map(|location| Violation {
                    kind: ViolationKind::LintSuppression,
                    location,
                })
        })
        .collect::<Result<Vec<Violation>, ModuleDependenciesError>>()
        .map(|suppressions| violations.extend(suppressions))
    }

    fn locate(
        &self,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
    ) -> Result<SourceLocation, ModuleDependenciesError> {
        self.locate_original(
            self.semantics
                .diagnostics_display_range_for_range(InFile::new(
                    self.semantics.hir_file_for(node),
                    node.text_range(),
                )),
            origin,
        )
    }

    fn locate_focus(
        &self,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
    ) -> Result<SourceLocation, ModuleDependenciesError> {
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
    ) -> Result<SourceLocation, ModuleDependenciesError> {
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

    fn misplaced_impl_kind(
        &self,
        home: &LocalImplHome,
        file: FileId,
    ) -> Result<ViolationKind, ModuleDependenciesError> {
        let database = self.semantics.db;

        self.locator.file(file).map(|home_file| match home {
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
        })
    }

    fn misplacement(
        &self,
        implementation: &ast::Impl,
        definition: Impl,
        origin: &SubtreeOrigin,
    ) -> Result<Option<Violation>, ModuleDependenciesError> {
        let database = self.semantics.db;
        let written_in = original_file_id(
            database,
            self.semantics.hir_file_for(implementation.syntax()),
        );

        self.locate_focus(implementation.syntax(), origin)
            .and_then(|location| {
                match (ImplementedType {
                    database,
                    implementation: definition,
                    krate: self.krate,
                })
                .home(&trait_arguments(implementation, self.semantics))
                {
                    ImplHome::Foreign => Ok(Some(Violation {
                        kind: ViolationKind::ImplWithoutLocalType,
                        location,
                    })),
                    ImplHome::Local { home } => home
                        .file(database)
                        .filter(|file| *file != written_in)
                        .map(|file| {
                            self.misplaced_impl_kind(&home, file)
                                .map(|kind| Violation { kind, location })
                        })
                        .transpose(),
                }
            })
    }

    fn module_item<'items>(
        &self,
        container: &SyntaxNode,
        public_items: &'items [ast::Item],
    ) -> Result<ModuleItem<'items>, ModuleDependenciesError> {
        self.semantics
            .scope(container)
            .map(|scope| self.module_name(scope.module()))
            .transpose()
            .map(|module_name| {
                module_name.flatten().map_or_else(
                    || {
                        public_items
                            .first()
                            .map_or(ModuleItem::Absent, ModuleItem::Accepted)
                    },
                    |module| {
                        public_items
                            .iter()
                            .find(|item| {
                                Self::item_name(item).is_some_and(|name| {
                                    NameWords { name: name.text() }.are_spelled_by(&module)
                                })
                            })
                            .map_or_else(
                                || {
                                    public_items.first().map_or(ModuleItem::Absent, |item| {
                                        ModuleItem::Misnamed { item, module }
                                    })
                                },
                                ModuleItem::Accepted,
                            )
                    },
                )
            })
    }

    fn module_name(&self, module: Module) -> Result<Option<String>, ModuleDependenciesError> {
        let database = self.semantics.db;

        module.name(database).map_or(Ok(None), |name| {
            if module.is_inline(database) {
                Ok(Some(name.as_str().to_owned()))
            } else {
                let file_id =
                    original_file_id(database, module.definition_source_file_id(database));

                self.locator
                    .source_file(file_id)
                    .map(|SourceFile { path, .. }| {
                        if module.is_mod_rs(database) {
                            path.parent().and_then(AbsPath::file_name)
                        } else {
                            path.file_stem()
                        }
                        .map(str::to_owned)
                    })
            }
        })
    }

    fn module_naming_violation(
        &self,
        module_item: ModuleItem<'_>,
        origin: &SubtreeOrigin,
    ) -> Result<Option<Violation>, ModuleDependenciesError> {
        match module_item {
            ModuleItem::Misnamed { item, module } => Self::item_name(item)
                .map(|name| {
                    let text = name.text();

                    self.locate_focus(item.syntax(), origin)
                        .map(|location| Violation {
                            kind: ViolationKind::ModuleNotNamedAfterItsItem {
                                expected: NameWords { name: text }.snake_case(),
                                item: text.to_owned(),
                                module,
                            },
                            location,
                        })
                })
                .transpose(),
            ModuleItem::Absent | ModuleItem::Accepted(_) => Ok(None),
        }
    }

    fn negated_attributes(
        &self,
        configured: &ConfiguredNode<'_>,
        origin: &SubtreeOrigin,
        ModuleFindings { violations, .. }: &mut ModuleFindings,
    ) -> Result<BTreeSet<SourceLocation>, ModuleDependenciesError> {
        configured
            .negated_attributes()
            .iter()
            .filter(|attribute| !self.token_origin.is_foreign_text(attribute.syntax()))
            .map(|attribute| self.locate(attribute.syntax(), origin))
            .collect::<Result<BTreeSet<SourceLocation>, ModuleDependenciesError>>()
            .inspect(|negations| {
                violations.extend(negations.iter().map(|location| Violation {
                    kind: ViolationKind::NegatedConfiguration,
                    location: location.clone(),
                }));
            })
    }

    fn non_doc_comments(
        &self,
        file_id: FileId,
        root: &SyntaxNode,
        ModuleFindings { violations, .. }: &mut ModuleFindings,
    ) -> Result<(), ModuleDependenciesError> {
        root.descendants_with_tokens()
            .filter_map(NodeOrToken::into_token)
            .filter_map(ast::Comment::cast)
            .map(|comment| {
                self.locator
                    .locate(FileRangeWrapper {
                        file_id,
                        range: comment.syntax().text_range(),
                    })
                    .map(|location| Violation {
                        kind: ViolationKind::NonDocComment,
                        location,
                    })
            })
            .collect::<Result<Vec<Violation>, ModuleDependenciesError>>()
            .map(|comments| violations.extend(comments))
    }

    fn order_violations<TKey: Ord>(
        &self,
        OrderedEntries { entries }: OrderedEntries<TKey>,
        kind: impl Fn(&TKey, &TKey, String) -> ViolationKind,
        origin: &SubtreeOrigin,
    ) -> Result<Vec<Violation>, ModuleDependenciesError> {
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
        .map(|MisorderedEntry { entry, preceding }| {
            self.locate_focus(&entry.node, origin)
                .map(|location| Violation {
                    kind: kind(
                        &entry.key,
                        &preceding.key,
                        ItemFocus {
                            node: &preceding.node,
                            semantics: self.semantics,
                        }
                        .text(),
                    ),
                    location,
                })
        })
        .collect()
    }

    fn own_violation(
        &self,
        kind: ViolationKind,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
    ) -> Result<Option<Violation>, ModuleDependenciesError> {
        self.token_origin
            .is_own_text(self.semantics.hir_file_for(node), node.text_range())
            .then(|| {
                self.locate(node, origin)
                    .map(|location| Violation { kind, location })
            })
            .transpose()
    }

    fn parameter_violations(
        &self,
        parameter: &ast::Param,
        origin: &SubtreeOrigin,
    ) -> Result<Vec<Violation>, ModuleDependenciesError> {
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
                .map(|kind| self.own_violation(kind, name.syntax(), origin))
            })
            .chain(iter::once(self.undestructured_parameter(parameter, origin)))
            .collect::<Result<Vec<Option<Violation>>, ModuleDependenciesError>>()
            .map(|violations| violations.into_iter().flatten().collect())
    }

    fn procedural_function(
        &self,
        function: Function,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
        usages: &DefinitionUsages<'_, '_>,
    ) -> Result<Option<Violation>, ModuleDependenciesError> {
        let database = self.semantics.db;

        function
            .params_without_self(database)
            .iter()
            .find_map(|parameter| {
                parameter.ty().autoderef(database).find_map(|dereferenced| {
                    dereferenced
                        .as_adt()
                        .filter(|adt| self.is_own_crate_adt(*adt))
                })
            })
            .filter(|_| !self.is_exempt_from_the_procedural_rule(function, node, usages))
            .map(|adt| {
                self.locate_focus(node, origin).map(|location| Violation {
                    kind: ViolationKind::ProceduralFunction {
                        type_name: adt
                            .name(database)
                            .display(database, self.krate.edition(database))
                            .to_string(),
                    },
                    location,
                })
            })
            .transpose()
    }

    fn push_expansion(
        &self,
        expansion: MacroCallId,
        call: &SyntaxNode,
        origin: &SubtreeOrigin,
        pending: &mut Vec<Subtree>,
        ModuleFindings { violations, .. }: &mut ModuleFindings,
    ) -> Result<(), ModuleDependenciesError> {
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

            Ok(())
        } else {
            self.locate(call, origin)
                .and_then(|location| self.unparsable(expansion, location, violations))
        }
    }

    fn reads_its_receiver(&self, function: Function) -> bool {
        let database = self.semantics.db;

        function
            .self_param(database)
            .and_then(|_| {
                function
                    .assoc_fn_params(database)
                    .first()
                    .and_then(|receiver| receiver.as_local(database))
            })
            .is_some_and(|receiver| {
                Definition::Local(receiver)
                    .usages(self.semantics)
                    .at_least_one()
            })
    }

    fn record(
        &self,
        path: &ast::Path,
        origin: &SubtreeOrigin,
        ModuleFindings {
            dependencies,
            unresolved_foreign_references,
            violations,
            ..
        }: &mut ModuleFindings,
    ) -> Result<(), ModuleDependenciesError> {
        match (PathReference {
            krate: self.krate,
            module_paths: self.module_paths,
            path,
            paths: self.paths,
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
                self.locate(path.syntax(), origin).map(|location| {
                    dependencies.insert(ModuleDependency {
                        from: referenced.from,
                        location,
                        to: referenced.to,
                    });
                })
            }
            PathReferenceOutcome::Irrelevant => Ok(()),
            PathReferenceOutcome::Unresolved => {
                self.locate(path.syntax(), origin).map(|location| {
                    violations.insert(Violation {
                        kind: ViolationKind::UnresolvedPath,
                        location,
                    });
                })
            }
            PathReferenceOutcome::UnresolvedInForeignModule { crate_name, module } => {
                self.locate(path.syntax(), origin).and_then(|location| {
                    self.locator
                        .file(original_file_id(
                            self.semantics.db,
                            module.definition_source_file_id(self.semantics.db),
                        ))
                        .map(|file| {
                            unresolved_foreign_references.insert(UnresolvedForeignReference {
                                location,
                                module: ForeignModule {
                                    file,
                                    name: iter::once(crate_name)
                                        .chain(
                                            self.module_paths
                                                .path(self.semantics.db, module)
                                                .segments,
                                        )
                                        .collect::<Vec<String>>()
                                        .join("::"),
                                },
                            });
                        })
                })
            }
        }
    }

    fn record_own_tokens(
        &self,
        file: HirFileId,
        tokens: &[WrittenToken],
        recorded: &mut WrittenTokens,
    ) -> Result<(), ModuleDependenciesError> {
        tokens
            .iter()
            .filter(|WrittenToken { token, written }| {
                self.token_origin
                    .is_own_written_text(file, token.text_range(), written)
            })
            .try_for_each(|WrittenToken { written, .. }| {
                self.locator.file(written.file_id).map(|written_file| {
                    recorded.insert(written_file, u32::from(written.range.start()));
                })
            })
    }

    fn referenced_test(
        &self,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
        usages: &DefinitionUsages<'_, '_>,
    ) -> Result<Option<Violation>, ModuleDependenciesError> {
        ast::Item::cast(node.clone())
            .filter(|item| {
                AttributedItem {
                    item,
                    paths: self.paths,
                    semantics: self.semantics,
                }
                .has_test_attribute()
                    && !usages.all().references.is_empty()
            })
            .map(|_| {
                self.locate_focus(node, origin).map(|location| Violation {
                    kind: ViolationKind::ReferencedTest,
                    location,
                })
            })
            .transpose()
    }

    fn signature_violations(
        &self,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
    ) -> Result<Vec<Violation>, ModuleDependenciesError> {
        ast::Param::cast(node.clone()).map_or_else(
            || {
                ast::TupleType::cast(node.clone())
                    .filter(|tuple| SignatureTuple { tuple }.is_in_a_signature())
                    .map_or_else(
                        || self.generic_parameter_violation(node, origin),
                        |tuple| {
                            self.own_violation(
                                ViolationKind::TupleInSignature,
                                tuple.syntax(),
                                origin,
                            )
                        },
                    )
                    .map(|violation| violation.into_iter().collect())
            },
            |parameter| self.parameter_violations(&parameter, origin),
        )
    }

    fn test_only_code(
        &self,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
    ) -> Result<Option<Violation>, ModuleDependenciesError> {
        (self.is_test_gated(node)
            && !self.token_origin.is_foreign_text(node)
            && !self.test_code.contains(node)
            && !Ancestry {
                node,
                semantics: self.semantics,
            }
            .enclosing()
            .is_some_and(|enclosing| self.in_test_gated_code(&enclosing)))
        .then(|| {
            self.locate_focus(node, origin).map(|location| Violation {
                kind: ViolationKind::TestOnlyCodeInProduction,
                location,
            })
        })
        .transpose()
    }

    fn test_target_violations(
        &self,
        items: &[ast::Item],
        origin: &SubtreeOrigin,
        ModuleFindings { violations, .. }: &mut ModuleFindings,
        TestTargetFindings {
            accepted,
            impl_blocks,
            types,
        }: &mut TestTargetFindings,
    ) -> Result<(), ModuleDependenciesError> {
        items.iter().try_for_each(|item| {
            self.locate_focus(item.syntax(), origin).map(|location| {
                let violation = Violation {
                    kind: ViolationKind::NonTestItemInTestTarget,
                    location,
                };

                if let ItemDefinition::ModuleDef(definition) =
                    ItemDefinition::new(item, self.semantics)
                    && (FileLocalItem {
                        file_id: original_file_id(
                            self.semantics.db,
                            self.semantics.hir_file_for(item.syntax()),
                        ),
                        usages: &DefinitionUsages::new(definition, self.semantics),
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
            })
        })
    }

    fn undestructured_parameter(
        &self,
        parameter: &ast::Param,
        origin: &SubtreeOrigin,
    ) -> Result<Option<Violation>, ModuleDependenciesError> {
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
                    krate: self.krate,
                    semantics: self.semantics,
                    token_origin: self.token_origin,
                }
                .destructurable_type()
                .map(|held| {
                    self.locate_focus(binding.syntax(), origin)
                        .map(|location| Violation {
                            kind: ViolationKind::UndestructuredParameter {
                                type_name: held
                                    .display(database, self.krate.to_display_target(database))
                                    .to_string(),
                            },
                            location,
                        })
                })
            })
            .transpose()
    }

    fn unparsable(
        &self,
        call: MacroCallId,
        location: SourceLocation,
        violations: &mut ViolationSet,
    ) -> Result<(), ModuleDependenciesError> {
        let violation = Violation {
            kind: ViolationKind::UnparsableMacroExpansion,
            location,
        };

        violations.insert(violation.clone());

        (ExpansionChain {
            call,
            locator: self.locator,
            semantics: self.semantics,
        })
        .findings()
        .map(|findings| {
            for finding in findings {
                violations.fold(violation.clone(), finding);
            }
        })
    }

    fn unresolved_captures(
        &self,
        node: &SyntaxNode,
        file: HirFileId,
        origin: &SubtreeOrigin,
        ModuleFindings { violations, .. }: &mut ModuleFindings,
    ) -> Result<(), ModuleDependenciesError> {
        code_tokens(node.children_with_tokens())
            .filter(|token| {
                ast::TokenTree::can_cast(node.kind())
                    && ast::String::can_cast(token.kind())
                    && self.token_origin.is_own_text(file, token.text_range())
                    && self.template_captures.captures(token).contains(&None)
            })
            .map(|token| {
                self.locate_original(
                    self.semantics
                        .diagnostics_display_range_for_range(InFile::new(file, token.text_range())),
                    origin,
                )
                .map(|location| Violation {
                    kind: ViolationKind::UnresolvedPath,
                    location,
                })
            })
            .collect::<Result<Vec<Violation>, ModuleDependenciesError>>()
            .map(|unresolved| violations.extend(unresolved))
    }

    fn unresolved_method_call(
        &self,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
    ) -> Result<Option<Violation>, ModuleDependenciesError> {
        ast::MethodCallExpr::cast(node.clone())
            .filter(|call| {
                self.is_unresolved_method_call(call)
                    && !call.receiver().is_some_and(|receiver| {
                        self.contains_unresolved_reference(receiver.syntax())
                    })
            })
            .and_then(|call| call.name_ref())
            .filter(|name| !self.token_origin.is_foreign_text(name.syntax()))
            .map(|name| {
                self.locate(name.syntax(), origin)
                    .map(|location| Violation {
                        kind: ViolationKind::UnresolvedMethodCall,
                        location,
                    })
            })
            .transpose()
    }

    fn unsafe_code(
        &self,
        node: &SyntaxNode,
        file: HirFileId,
        origin: &SubtreeOrigin,
        UnsafeCodeFindings {
            integrates_foreign_code,
            sites,
        }: &mut UnsafeCodeFindings,
    ) -> Result<(), ModuleDependenciesError> {
        code_tokens(node.children_with_tokens())
            .filter(|token| {
                token.kind() == T![unsafe]
                    && !ast::FnPtrType::can_cast(node.kind())
                    && !ast::TokenTree::can_cast(node.kind())
                    && self.token_origin.is_own_text(file, token.text_range())
            })
            .map(|token| {
                self.locate_original(
                    self.semantics
                        .diagnostics_display_range_for_range(InFile::new(file, token.text_range())),
                    origin,
                )
                .map(|location| Violation {
                    kind: ViolationKind::UnsafeCode,
                    location,
                })
            })
            .collect::<Result<Vec<Violation>, ModuleDependenciesError>>()
            .map(|unsafe_sites| {
                sites.extend(unsafe_sites);
                *integrates_foreign_code =
                    *integrates_foreign_code || self.integrates_foreign_code(node, file);
            })
    }

    fn visit(
        &self,
        node: &SyntaxNode,
        definition: &ItemDefinition,
        origin: &SubtreeOrigin,
        pending: &mut Vec<Subtree>,
        WalkedModule {
            crate_findings,
            findings,
            ..
        }: &mut WalkedModule,
    ) -> Result<(), ModuleDependenciesError> {
        self.unresolved_method_call(node, origin)
            .and_then(|unresolved| {
                findings.violations.extend(unresolved);

                if let Some(path) = ast::Path::cast(node.clone()) {
                    self.record(&path, origin, findings)
                } else if let Some(call) = ast::MacroCall::cast(node.clone()) {
                    self.expand(&call, origin, pending, findings)
                } else if let Some(derive) = ast::Meta::cast(node.clone()) {
                    self.expand_derives(&derive, origin, pending, findings)
                } else if let ItemDefinition::Module(module) = definition
                    && let Some(declaration) = ast::Module::cast(node.clone())
                        .filter(|declaration| declaration.item_list().is_none())
                {
                    self.declare(&declaration, *module, origin, findings)
                } else if let ItemDefinition::Impl(implementation) = definition
                    && let Some(written) = ast::Impl::cast(node.clone())
                        .filter(|written| !self.token_origin.is_foreign_text(written.syntax()))
                {
                    self.implementation(&written, *implementation, origin, findings)
                } else if self.is_reexport(node, definition) {
                    self.locate_focus(node, origin).map(|location| {
                        findings.violations.insert(Violation {
                            kind: ViolationKind::Reexport,
                            location,
                        });
                    })
                } else if let Some(tree) = ast::UseTree::cast(node.clone()).filter(|tree| {
                    tree.star_token().is_some_and(|star| {
                        self.token_origin.is_own_text(
                            self.semantics.hir_file_for(tree.syntax()),
                            star.text_range(),
                        )
                    })
                }) {
                    self.locate(tree.syntax(), origin).map(|location| {
                        findings.violations.insert(Violation {
                            kind: ViolationKind::GlobImport,
                            location,
                        });
                    })
                } else if matches!(origin, SubtreeOrigin::SourceFile)
                    && let ItemDefinition::Const(constant) = definition
                    && let Some(kind) = self.misplaced_constant(*constant)
                {
                    self.locate_focus(node, origin).map(|location| {
                        findings.violations.insert(Violation { kind, location });
                    })
                } else if matches!(origin, SubtreeOrigin::SourceFile)
                    && let ItemDefinition::ModuleDef(ModuleDef::Function(function)) = definition
                {
                    self.function(*function, node, origin, findings)
                } else if matches!(origin, SubtreeOrigin::SourceFile)
                    && let ItemDefinition::ModuleDef(ModuleDef::Trait(declaring)) = definition
                    && let Some(items) =
                        ast::Trait::cast(node.clone()).and_then(|written| written.assoc_item_list())
                {
                    self.grouped_method_order_violations(
                        &items,
                        MethodOwner::TraitDeclaration(*declaring),
                        origin,
                    )
                    .map(|misordered| findings.violations.extend(misordered))
                } else if matches!(origin, SubtreeOrigin::SourceFile)
                    && Self::is_public_item_container(node)
                {
                    self.container(node, origin, findings, crate_findings)
                } else {
                    self.signature_violations(node, origin)
                        .map(|violations| findings.violations.extend(violations))
                }
            })
    }

    fn walk(
        &self,
        root: Subtree,
        walked: &mut WalkedModule,
    ) -> Result<(), ModuleDependenciesError> {
        let mut pending = vec![root];
        let mut outcome = Ok(());

        while outcome.is_ok()
            && let Some(Subtree { origin, root }) = pending.pop()
        {
            let file = self.semantics.hir_file_for(&root);
            let mut preorder = root.preorder();

            while outcome.is_ok()
                && let Some(event) = preorder.next()
            {
                if let WalkEvent::Enter(node) = event {
                    outcome = self.enter(&node, file, &origin, &mut preorder, &mut pending, walked);
                }
            }
        }

        outcome
    }

    fn written_tokens(
        &self,
        file: HirFileId,
        tokens: impl Iterator<Item = SyntaxToken>,
    ) -> Vec<WrittenToken> {
        tokens
            .map(|token| WrittenToken {
                written: written_range(self.semantics.db, file, token.text_range()),
                token,
            })
            .collect()
    }
}
