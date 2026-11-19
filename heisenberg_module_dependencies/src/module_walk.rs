use std::cell::Cell;
use std::cell::OnceCell;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter;
use std::sync::Arc;

use ra_ap_cfg::CfgAtom;
use ra_ap_cfg::CfgExpr;
use ra_ap_cfg::CfgOptions;
use ra_ap_hir::Adt;
use ra_ap_hir::AsAssocItem;
use ra_ap_hir::AssocItem;
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
use ra_ap_hir::TypeAlias;
use ra_ap_hir::Variant;
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
use ra_ap_ide_db::base_db::Crate as DatabaseCrate;
use ra_ap_ide_db::defs::Definition;
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
use ra_ap_syntax::WalkEvent;
use ra_ap_syntax::ast;
use ra_ap_syntax::ast::HasGenericArgs;
use ra_ap_syntax::ast::HasName as _;
use ra_ap_syntax::ast::HasVisibility as _;
use ra_ap_vfs::FileId;

use heisenberg_configured_code::ancestry::Ancestry;
use heisenberg_configured_code::conditional_attribute::ConditionalAttribute;
use heisenberg_configured_code::configured_node::ConfiguredNode;
use heisenberg_configured_code::negated_atom::NegatedAtom;
use heisenberg_configured_code::negated_configuration::NegatedConfiguration;
use heisenberg_configured_code::non_production_code::NonProductionCode;
use heisenberg_configured_code::test_predicate::TestPredicate;
use heisenberg_expanded_code::code_tokens::code_tokens;
use heisenberg_module_graph::code_category::CodeCategory;
use heisenberg_module_graph::configuration_atom::ConfigurationAtom;
use heisenberg_module_graph::configured_out_negations::ConfiguredOutNegations;
use heisenberg_module_graph::crate_target::CrateTarget;
use heisenberg_module_graph::crate_target_kind::CrateTargetKind;
use heisenberg_module_graph::dependency_reference::DependencyReference;
use heisenberg_module_graph::error_issuance::ErrorIssuance;
use heisenberg_module_graph::error_violation::ErrorViolation;
use heisenberg_module_graph::every_configuration_findings::EveryConfigurationFindings;
use heisenberg_module_graph::foreign_module::ForeignModule;
use heisenberg_module_graph::harness_violation::HarnessViolation;
use heisenberg_module_graph::impl_placement::ImplPlacement;
use heisenberg_module_graph::inactive_focus::InactiveFocus;
use heisenberg_module_graph::item_group::ItemGroup;
use heisenberg_module_graph::item_placement::ItemPlacement;
use heisenberg_module_graph::layout_violation::LayoutViolation;
use heisenberg_module_graph::method_placement::MethodPlacement;
use heisenberg_module_graph::module_dependency::ModuleDependency;
use heisenberg_module_graph::module_file_analysis::ModuleFileAnalysis;
use heisenberg_module_graph::module_findings::ModuleFindings;
use heisenberg_module_graph::non_production_users::NonProductionUsers;
use heisenberg_module_graph::placement_violation::PlacementViolation;
use heisenberg_module_graph::rule_applicability::RuleApplicability;
use heisenberg_module_graph::shared_module_file::SharedModuleFile;
use heisenberg_module_graph::source_location::SourceLocation;
use heisenberg_module_graph::test_violation::TestViolation;
use heisenberg_module_graph::unresolved_foreign_reference::UnresolvedForeignReference;
use heisenberg_module_graph::untyped_error_name::UntypedErrorName;
use heisenberg_module_graph::used_item::UsedItem;
use heisenberg_module_graph::violation::Violation;
use heisenberg_module_graph::violation_kind::ViolationKind;
use heisenberg_module_graph::violation_set::ViolationSet;
use heisenberg_module_graph::walked_code::WalkedCode;
use heisenberg_module_graph::written_tokens::WrittenTokens;
use heisenberg_project_model::constants::BENCHMARK_LOOP_OUTPUT_PARAMETER;
use heisenberg_project_model::constants::BENCHMARK_REGISTRATIONS;
use heisenberg_project_model::constants::OUTPUT_DROPPING_BENCHMARK_LOOPS;
use heisenberg_project_model::constants::VETTED_BENCHMARK_HARNESS_METHODS;
use heisenberg_project_model::source_file::SourceFile;
use heisenberg_source_locator::original_file_id::original_file_id;
use heisenberg_source_locator::source_locator::SourceLocator;
use heisenberg_source_locator::written_range::written_range;
use heisenberg_task_graph::once_map::OnceMap;

use crate::adt_violation::AdtViolation;
use crate::attribute_directives::AttributeDirectives;
use crate::attributed_item::AttributedItem;
use crate::benchmark_harness::BenchmarkHarness;
use crate::benchmark_output::BenchmarkOutput;
use crate::builtin_attribute::BuiltinAttribute;
use crate::cargo_target_variables::CargoTargetVariables;
use crate::constants::LIFETIME_PREFIX;
use crate::constants::TEST_MODULE_NAME;
use crate::constants::UNSAFE_ATTRIBUTES;
use crate::constants_module::ConstantsModule;
use crate::constructor::Constructor;
use crate::covered_code::CoveredCode;
use crate::crate_findings::CrateFindings;
use crate::defined_items::DefinedItems;
use crate::definition_usages::DefinitionUsages;
use crate::derived_sources::DerivedSources;
use crate::error_boundary::ErrorBoundary;
use crate::error_derive::ErrorDerive;
use crate::error_implementation_origin::ErrorImplementationOrigin;
use crate::error_types::ErrorTypes;
use crate::expanded_items::ExpandedItems;
use crate::expansion_chain::ExpansionChain;
use crate::field_only_parameter::FieldOnlyParameter;
use crate::file_local_item::FileLocalItem;
use crate::file_start::file_start;
use crate::function_bodies::FunctionBodies;
use crate::impl_home::ImplHome;
use crate::implemented_type::ImplementedType;
use crate::is_module_inside_block::is_module_inside_block;
use crate::item_adt::ItemAdt;
use crate::item_definition::ItemDefinition;
use crate::item_expansion::ItemExpansion;
use crate::item_focus::ItemFocus;
use crate::item_order::ItemOrder;
use crate::item_references::ItemReferences;
use crate::local_impl_home::LocalImplHome;
use crate::local_macro::LocalMacro;
use crate::local_macro_import::LocalMacroImport;
use crate::macro_expansion::MacroExpansion;
use crate::method_call_resolutions::MethodCallResolutions;
use crate::method_grouping::MethodGrouping;
use crate::method_key::MethodKey;
use crate::method_order::MethodOrder;
use crate::method_owner::MethodOwner;
use crate::misordered_entry::MisorderedEntry;
use crate::module_dependencies_error::ModuleDependenciesError;
use crate::module_item::ModuleItem;
use crate::module_paths::ModulePaths;
use crate::name_words::NameWords;
use crate::node_attributes::NodeAttributes;
use crate::ordered_entries::OrderedEntries;
use crate::ordered_entry::OrderedEntry;
use crate::output_use::OutputUse;
use crate::outside_visibility::OutsideVisibility;
use crate::parameter_name::ParameterName;
use crate::path_position::PathPosition;
use crate::path_reference::PathReference;
use crate::path_reference_outcome::PathReferenceOutcome;
use crate::path_resolutions::PathResolutions;
use crate::public_items::PublicItems;
use crate::qualified_names::QualifiedNames;
use crate::signature_tuple::SignatureTuple;
use crate::standard_items::StandardItems;
use crate::substituted_resolution::SubstitutedResolution;
use crate::subtree::Subtree;
use crate::subtree_origin::SubtreeOrigin;
use crate::template_captures::TemplateCaptures;
use crate::test_target_findings::TestTargetFindings;
use crate::test_target_items::TestTargetItems;
use crate::token_origin::TokenOrigin;
use crate::tool_directive::ToolDirective;
use crate::trait_arguments::trait_arguments;
use crate::unsafe_code_findings::UnsafeCodeFindings;
use crate::unsafe_code_use::UnsafeCodeUse;
use crate::used_definition::UsedDefinition;
use crate::walked_module::WalkedModule;
use crate::written_result_error::WrittenResultError;
use crate::written_token::WrittenToken;

pub struct ModuleWalk<'walk, 'database> {
    pub benchmark_harness: &'walk BenchmarkHarness<'walk, 'database>,
    pub build_script_output_files: &'walk BTreeSet<FileId>,
    pub cargo_target_variables: &'walk CargoTargetVariables,
    pub crate_target: &'walk CrateTarget,
    pub dependency_names: &'walk HashMap<Crate, String>,
    pub error_derive_crates: &'walk HashSet<Crate>,
    pub error_runtime_crates: &'walk HashSet<Crate>,
    pub file_modules: &'walk HashMap<FileId, Vec<Module>>,
    pub inactive_code: &'walk HashMap<InFile<SyntaxNodePtr>, CfgExpr>,
    pub krate: Crate,
    pub locator: &'walk SourceLocator<'walk>,
    pub method_calls: &'walk MethodCallResolutions<'walk, 'database>,
    pub module_paths: &'walk ModulePaths,
    pub non_production_code: &'walk NonProductionCode<'walk, 'database>,
    pub options: &'walk CfgOptions,
    pub paths: &'walk PathResolutions<'walk, 'database>,
    pub production_options: &'walk CfgOptions,
    pub runs_libtest_tests: &'walk Cell<bool>,
    pub semantics: &'walk Semantics<'database, RootDatabase>,
    pub standard_items: &'walk StandardItems,
    pub template_captures: &'walk TemplateCaptures<'walk, 'database>,
    pub test_gated_code: &'walk OnceMap<InFile<SyntaxNodePtr>, bool>,
    pub token_origin: &'walk TokenOrigin<'walk, 'database>,
    pub trait_result_errors:
        &'walk OnceMap<Trait, Result<Arc<HashSet<Name>>, Arc<ModuleDependenciesError>>>,
    pub unpublished_packages: &'walk BTreeSet<String>,
    pub usages: &'walk DefinitionUsages<'walk, 'database>,
}

impl<'database> ModuleWalk<'_, 'database> {
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

                file_start(self.locator, file_id).and_then(|SourceLocation { file, .. }| {
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
                                    paths: self.paths,
                                    semantics: self.semantics,
                                    template_captures: self.template_captures,
                                }
                                .written_in(&root)
                                .map(|bodies| walked.function_bodies.extend(bodies))
                            })
                            .map(|()| {
                                walked.crate_findings.runs_libtest_tests |=
                                    self.runs_libtest_tests.get();
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

    fn attribute_directives(
        &self,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
        ModuleFindings { violations, .. }: &mut ModuleFindings,
    ) -> Result<(), ModuleDependenciesError> {
        let holds_workspace_text = OnceCell::new();
        let AttributeDirectives {
            ignores,
            suppressions,
            tool_attributes,
        } = NodeAttributes {
            node,
            options: self.options,
            paths: self.paths,
            semantics: self.semantics,
        }
        .directives();

        self.record_test(node);

        suppressions
            .iter()
            .filter(|meta| {
                self.is_written_by_the_workspace(meta.syntax())
                    && *holds_workspace_text.get_or_init(|| {
                        CoveredCode {
                            options: self.options,
                            semantics: self.semantics,
                            token_origin: self.token_origin,
                        }
                        .holds_workspace_text(node)
                    })
            })
            .map(|meta| self.attribute_violation(meta, ViolationKind::LintSuppression, origin))
            .chain(
                ignores
                    .iter()
                    .filter(|meta| self.is_written_by_the_workspace(meta.syntax()))
                    .map(|meta| {
                        self.attribute_violation(
                            meta,
                            ViolationKind::Test(TestViolation::Ignored),
                            origin,
                        )
                    }),
            )
            .chain(
                tool_attributes
                    .iter()
                    .filter(|ToolDirective { meta, .. }| {
                        self.is_written_by_the_workspace(meta.syntax())
                    })
                    .map(|ToolDirective { meta, tool }| {
                        self.attribute_violation(
                            meta,
                            ViolationKind::ToolAttribute { tool: tool.clone() },
                            origin,
                        )
                    }),
            )
            .collect::<Result<Vec<Violation>, ModuleDependenciesError>>()
            .map(|found| violations.extend(found))
    }

    fn attribute_violation(
        &self,
        meta: &ast::Meta,
        kind: ViolationKind,
        origin: &SubtreeOrigin,
    ) -> Result<Violation, ModuleDependenciesError> {
        self.locate(meta.syntax(), origin)
            .map(|location| Violation { kind, location })
    }

    fn checks_exporting_uses(
        &self,
        definition: Definition<'database>,
        item: UsedItem,
        node: &SyntaxNode,
    ) -> bool {
        item != UsedItem::Variant
            && self
                .unpublished_packages
                .contains(&self.crate_target.package_name)
            && self.is_exported(definition)
            && self.placement_rules_apply(node)
    }

    fn conditional_attributes(
        &self,
        configured: &ConfiguredNode<'_>,
        origin: &SubtreeOrigin,
        ModuleFindings {
            every_configuration: EveryConfigurationFindings { held, refuted },
            ..
        }: &mut ModuleFindings,
    ) -> Result<(), ModuleDependenciesError> {
        let in_production = ConfiguredNode {
            node: configured.node,
            options: self.production_options,
        }
        .conditional_attributes();

        configured
            .conditional_attributes()
            .into_iter()
            .zip(in_production)
            .filter(|(ConditionalAttribute { meta, .. }, _)| {
                !self.token_origin.is_foreign_text(meta.syntax())
            })
            .try_for_each(|(under_test, in_production)| {
                self.locate(under_test.meta.syntax(), origin)
                    .map(|location| {
                        let attribute = Violation {
                            kind: ViolationKind::AttributeAppliedInNoConfiguration,
                            location,
                        };

                        if under_test.applies || in_production.applies {
                            refuted.insert(attribute);
                        } else {
                            held.insert(attribute);
                        }
                    })
            })
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

    fn constructed_error(
        &self,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
    ) -> Result<Option<Violation>, ModuleDependenciesError> {
        ast::Path::cast(node.clone())
            .filter(|path| PathPosition { path }.construction().is_some())
            .and_then(|path| match self.paths.resolve(&path) {
                Some(PathResolution::Def(ModuleDef::EnumVariant(variant))) => self
                    .error_boundary()
                    .outside_owner_of_variant(variant)
                    .zip(Some(path)),
                _ => None,
            })
            .map_or(Ok(None), |(owner, path)| {
                self.qualified_names().crate_name(owner).and_then(|owner| {
                    self.own_violation(
                        ViolationKind::Error(ErrorViolation::IssuedOutsideItsCrate {
                            issuance: ErrorIssuance::Constructed {
                                written: path.syntax().to_string(),
                            },
                            owner,
                        }),
                        path.syntax(),
                        origin,
                    )
                })
            })
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
                        &ViolationKind::Placement(PlacementViolation::NonConstantInConstantsModule),
                        origin,
                    )
                    .map(|non_constants| module_findings.violations.extend(non_constants))
                } else {
                    self.module_item(node, &non_constant)
                        .and_then(|module_item| {
                            let accepted = module_item.item();

                            self.item_violations(
                                non_constant.iter().filter(|item| Some(*item) != accepted),
                                &ViolationKind::Placement(PlacementViolation::AdditionalPublicItem),
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
                match self.crate_target.kind {
                    CrateTargetKind::Benchmark => {
                        Some(ViolationKind::NonBenchmarkItemInBenchmarkTarget)
                    }
                    CrateTargetKind::IntegrationTest => {
                        Some(ViolationKind::NonTestItemInTestTarget)
                    }
                    CrateTargetKind::Binary
                    | CrateTargetKind::BuildScript
                    | CrateTargetKind::Example
                    | CrateTargetKind::Library => None,
                }
                .map_or(Ok(()), |kind| {
                    self.test_target_violations(
                        &kind,
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
                })
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

    fn dependency_reference(
        &self,
        referenced: Option<Crate>,
        node: &SyntaxNode,
    ) -> Option<DependencyReference> {
        referenced
            .and_then(|krate| self.dependency_names.get(&krate))
            .map(|extern_crate_name| DependencyReference {
                category: if self.non_production_code.is_benchmark(node) {
                    CodeCategory::Benchmark
                } else if self.is_test_code(node) {
                    CodeCategory::Test
                } else {
                    CodeCategory::Production
                },
                extern_crate_name: extern_crate_name.clone(),
            })
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

        self.vacuous_negations(&configured, origin, &mut walked.findings)
            .and_then(|()| self.conditional_attributes(&configured, origin, &mut walked.findings))
            .and_then(|()| self.negated_attributes(&configured, origin, &mut walked.findings))
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
                        .and_then(|()| {
                            self.attribute_directives(node, origin, &mut walked.findings)
                        })
                        .and_then(|()| self.test_only_code(node, origin))
                        .and_then(|test_only| {
                            walked.findings.violations.extend(test_only);
                            self.test_outside_a_test_module(node, origin)
                        })
                        .and_then(|misplaced_test| {
                            walked.findings.violations.extend(misplaced_test);
                            self.items_used_outside_production(
                                node,
                                origin,
                                &mut walked.findings.every_configuration,
                            )
                        })
                        .and_then(|used_outside_production| {
                            walked.findings.violations.extend(used_outside_production);
                            self.item(node, origin, preorder, pending, walked)
                        })
                }
            })
    }

    const fn error_boundary(&self) -> ErrorBoundary<'_, 'database> {
        ErrorBoundary {
            error_types: self.error_types(),
            krate: self.krate,
            token_origin: self.token_origin,
        }
    }

    const fn error_derive(&self) -> ErrorDerive<'_, 'database> {
        ErrorDerive {
            derive_crates: self.error_derive_crates,
            error_types: self.error_types(),
            method_calls: self.method_calls,
            paths: self.paths,
            runtime_crates: self.error_runtime_crates,
            semantics: self.semantics,
        }
    }

    fn error_derive_violations(
        &self,
        expansion: MacroCallId,
        derive: &ast::Meta,
        origin: &SubtreeOrigin,
        ModuleFindings { violations, .. }: &mut ModuleFindings,
    ) -> Result<(), ModuleDependenciesError> {
        let error_derive = self.error_derive();

        match derive.syntax().ancestors().find_map(ast::Enum::cast) {
            Some(written)
                if error_derive.generates(expansion)
                    && self.error_rules_apply(written.syntax()) =>
            {
                let DerivedSources {
                    outside_chain,
                    sources,
                } = error_derive.sources(&written, expansion);
                let expansion_origin = origin.expansion_of(derive.syntax());

                outside_chain
                    .iter()
                    .try_for_each(|field| {
                        self.locate_focus(field, origin).map(|location| {
                            violations.insert(Violation {
                                kind: ViolationKind::Error(ErrorViolation::SourceOutsideChain),
                                location,
                            });
                        })
                    })
                    .and_then(|()| {
                        self.error_message_violations(&written, origin)
                            .map(|misplaced| violations.extend(misplaced))
                    })
                    .and_then(|()| {
                        self.error_module_violation(&written, origin)
                            .map(|misplaced| violations.extend(misplaced))
                    })
                    .and_then(|()| {
                        error_derive
                            .sources_in_messages(expansion, &sources)
                            .iter()
                            .try_for_each(|argument| {
                                self.locate(argument, &expansion_origin).map(|location| {
                                    violations.insert(Violation {
                                        kind: ViolationKind::Error(ErrorViolation::SourceInMessage),
                                        location,
                                    });
                                })
                            })
                    })
            }
            _ => Ok(()),
        }
    }

    fn error_display(
        &self,
        node: &SyntaxNode,
        definition: &ItemDefinition,
        origin: &SubtreeOrigin,
    ) -> Result<Option<Violation>, ModuleDependenciesError> {
        let database = self.semantics.db;

        match definition {
            ItemDefinition::Impl(implementation)
                if implementation.trait_(database) == Some(self.standard_items.display)
                    && self.semantics.hir_file_for(node).macro_file().is_none()
                    && self
                        .error_types()
                        .implements_error(&implementation.self_ty(database))
                    && self.error_rules_apply(node) =>
            {
                self.locate_focus(node, origin).map(|location| {
                    Some(Violation {
                        kind: ViolationKind::Error(ErrorViolation::HandwrittenDisplay),
                        location,
                    })
                })
            }
            _ => Ok(None),
        }
    }

    fn error_implementation(
        &self,
        node: &SyntaxNode,
        definition: &ItemDefinition,
        origin: &SubtreeOrigin,
        ModuleFindings { violations, .. }: &mut ModuleFindings,
        CrateFindings { error_types, .. }: &mut CrateFindings,
    ) -> Result<(), ModuleDependenciesError> {
        match definition {
            ItemDefinition::Impl(implementation)
                if implementation.trait_(self.semantics.db)
                    == Some(self.standard_items.error_trait) =>
            {
                self.locate_focus(node, origin).map(|location| {
                    let origin_violation = match self
                        .error_implementation_origin(node, *implementation)
                    {
                        ErrorImplementationOrigin::DerivedForAnEnum => None,
                        ErrorImplementationOrigin::DerivedForAnotherType => {
                            Some(ErrorViolation::NotAnEnum)
                        }
                        ErrorImplementationOrigin::Handwritten => Some(ErrorViolation::Handwritten),
                        ErrorImplementationOrigin::MacroGenerated => {
                            Some(ErrorViolation::MacroGenerated)
                        }
                    };

                    if self.error_rules_apply(node) {
                        violations.extend(origin_violation.map(|violation| Violation {
                            kind: ViolationKind::Error(violation),
                            location: location.clone(),
                        }));

                        error_types
                            .implementations
                            .insert(*implementation, location);
                    }
                })
            }
            _ => Ok(()),
        }
    }

    fn error_implementation_origin(
        &self,
        node: &SyntaxNode,
        implementation: Impl,
    ) -> ErrorImplementationOrigin {
        match self.semantics.hir_file_for(node).macro_file() {
            None => ErrorImplementationOrigin::Handwritten,
            Some(expansion) if !self.error_derive().generates(expansion) => {
                ErrorImplementationOrigin::MacroGenerated
            }
            Some(_) => match implementation.self_ty(self.semantics.db).as_adt() {
                Some(Adt::Enum(_)) => ErrorImplementationOrigin::DerivedForAnEnum,
                _ => ErrorImplementationOrigin::DerivedForAnotherType,
            },
        }
    }

    fn error_message_violations(
        &self,
        written: &ast::Enum,
        origin: &SubtreeOrigin,
    ) -> Result<Vec<Violation>, ModuleDependenciesError> {
        let error_derive = self.error_derive();

        error_derive
            .enum_messages(written)
            .iter()
            .map(|attribute| {
                self.locate(attribute.syntax(), origin)
                    .map(|location| Violation {
                        kind: ViolationKind::Error(ErrorViolation::MessageOnTheEnum),
                        location,
                    })
            })
            .chain(
                error_derive
                    .message_functions(written)
                    .iter()
                    .map(|attribute| {
                        self.locate(attribute.syntax(), origin)
                            .map(|location| Violation {
                                kind: ViolationKind::Error(ErrorViolation::MessageFunction),
                                location,
                            })
                    }),
            )
            .collect()
    }

    fn error_module_violation(
        &self,
        written: &ast::Enum,
        origin: &SubtreeOrigin,
    ) -> Result<Option<Violation>, ModuleDependenciesError> {
        let database = self.semantics.db;

        self.semantics
            .to_def(written)
            .zip(written.name())
            .map_or(Ok(None), |(definition, name)| {
                let module = definition.module(database);

                self.module_name(module).and_then(|module_name| {
                    let home = module_name.filter(|_| !module.is_inline(database));

                    if home.as_ref().is_some_and(|file_module| {
                        NameWords { name: name.text() }.are_spelled_by(file_module)
                    }) {
                        Ok(None)
                    } else {
                        self.locate_focus(written.syntax(), origin).map(|location| {
                            Some(Violation {
                                kind: ViolationKind::Error(ErrorViolation::OutsideItsModule {
                                    module: self.module_paths.path(database, module),
                                }),
                                location,
                            })
                        })
                    }
                })
            })
    }

    fn error_rules_apply(&self, node: &SyntaxNode) -> bool {
        self.is_applicable(
            ViolationKind::Error(ErrorViolation::Handwritten).applicability(),
            node,
        )
    }

    const fn error_types(&self) -> ErrorTypes<'_, 'database> {
        ErrorTypes {
            semantics: self.semantics,
            standard_items: self.standard_items,
        }
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
                .and_then(|()| self.undeclared_environment_variable(call, origin))
                .and_then(|undeclared| {
                    module_findings.violations.extend(undeclared);
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
                self.error_derive_violations(expansion, derive, origin, module_findings)
                    .and_then(|()| {
                        self.push_expansion(
                            expansion,
                            derive.syntax(),
                            origin,
                            pending,
                            module_findings,
                        )
                    })
            })
    }

    fn export_finding(
        &self,
        definition: Definition<'database>,
        item: UsedItem,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
        EveryConfigurationFindings { held, refuted }: &mut EveryConfigurationFindings,
        exporting_uses_checked: bool,
    ) -> Result<(), ModuleDependenciesError> {
        if exporting_uses_checked {
            let is_needed = ItemReferences {
                definition,
                item,
                node,
                semantics: self.semantics,
                usages: self.usages,
            }
            .exporting_uses()
            .iter()
            .any(|user| {
                !self.token_origin.is_own_file(original_file_id(
                    self.semantics.db,
                    self.semantics.hir_file_for(user),
                )) || !user.ancestors().any(|ancestor| {
                    ast::BlockExpr::can_cast(ancestor.kind()) || ast::Use::can_cast(ancestor.kind())
                })
            });

            self.locate_focus(node, origin).map(|location| {
                let finding = Violation {
                    kind: ViolationKind::Placement(PlacementViolation::MoreVisibleThanItsUses {
                        item,
                    }),
                    location,
                };

                if is_needed {
                    refuted.insert(finding);
                } else {
                    held.insert(finding);
                }
            })
        } else {
            Ok(())
        }
    }

    fn function(
        &self,
        function: Function,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
        ModuleFindings {
            every_configuration,
            violations,
            ..
        }: &mut ModuleFindings,
    ) -> Result<(), ModuleDependenciesError> {
        let database = self.semantics.db;
        let is_trait_item = function
            .as_assoc_item(database)
            .is_some_and(|item| item.container_or_implemented_trait(database).is_some());

        let exporting_uses_checked = !is_trait_item
            && self.checks_exporting_uses(Definition::Function(function), UsedItem::Function, node);

        self.procedural_function(function, node, origin, is_trait_item)
            .and_then(|procedural| {
                violations.extend(procedural);
                if is_trait_item {
                    Ok(None)
                } else {
                    self.item_used_outside_production(
                        Definition::Function(function),
                        UsedItem::Function,
                        node,
                        origin,
                        exporting_uses_checked,
                    )
                }
            })
            .and_then(|used_outside_production| {
                violations.extend(used_outside_production);
                self.referenced_test(function, node, origin)
            })
            .and_then(|referenced| {
                violations.extend(referenced);
                if is_trait_item {
                    Ok(())
                } else {
                    self.export_finding(
                        Definition::Function(function),
                        UsedItem::Function,
                        node,
                        origin,
                        every_configuration,
                        exporting_uses_checked,
                    )
                }
            })
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

    fn harness_call(
        &self,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
        findings: &mut ModuleFindings,
        crate_findings: &mut CrateFindings,
    ) -> Result<(), ModuleDependenciesError> {
        if let Some(call) = ast::MethodCallExpr::cast(node.clone())
            && let Some(resolved) = self.method_calls.resolve(&call)
            && let Some(name) = call.name_ref()
        {
            self.harness_method_use(&resolved, name.syntax(), origin, findings, crate_findings)
        } else if let Some(path) = ast::Path::cast(node.clone())
            && let Some(SubstitutedResolution {
                resolution: PathResolution::Def(ModuleDef::Function(function)),
                substitution,
            }) = self.paths.resolve_substituted(&path)
            && let Some(name) = path.segment().and_then(|segment| segment.name_ref())
        {
            self.harness_method_use(
                &SubstitutedResolution {
                    resolution: function,
                    substitution,
                },
                name.syntax(),
                origin,
                findings,
                crate_findings,
            )
        } else {
            Ok(())
        }
    }

    fn harness_method_use(
        &self,
        called: &SubstitutedResolution<'database, Function>,
        name: &SyntaxNode,
        origin: &SubtreeOrigin,
        ModuleFindings { violations, .. }: &mut ModuleFindings,
        CrateFindings {
            registers_benchmark,
            ..
        }: &mut CrateFindings,
    ) -> Result<(), ModuleDependenciesError> {
        self.benchmark_harness
            .method(called.resolution)
            .map_or(Ok(None), |method| {
                *registers_benchmark =
                    *registers_benchmark || method.is_listed_in(&BENCHMARK_REGISTRATIONS);

                if !method.is_listed_in(&VETTED_BENCHMARK_HARNESS_METHODS) {
                    Ok(Some(ViolationKind::BenchmarkHarness(
                        HarnessViolation::UnvettedMethod {
                            method: method.to_string(),
                        },
                    )))
                } else if method.is_listed_in(&OUTPUT_DROPPING_BENCHMARK_LOOPS) {
                    self.loop_output_violation(called)
                } else {
                    Ok(None)
                }
            })
            .and_then(|kind| kind.map_or(Ok(None), |kind| self.own_violation(kind, name, origin)))
            .map(|violation| violations.extend(violation))
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

        (ast::ExternBlock::cast(node.clone())
            .and_then(|block| block.extern_item_list())
            .is_some_and(|items| items.extern_items().next().is_some())
            && self.token_origin.is_workspace_code_at(
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
                            .is_workspace_code_at(file, abi.syntax().text_range())
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
                        .is_workspace_code_at(file, path.syntax().text_range())
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

    fn is_applicable(&self, applicability: RuleApplicability, node: &SyntaxNode) -> bool {
        match applicability {
            RuleApplicability::AllCode => true,
            RuleApplicability::ProductionCode => !self.is_test_code(node),
        }
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

    fn is_exempt_from_the_procedural_rule(&self, function: Function, node: &SyntaxNode) -> bool {
        let database = self.semantics.db;

        function
            .as_assoc_item(database)
            .and_then(|item| item.implementing_ty(database))
            .is_some_and(|self_type| {
                Constructor {
                    function,
                    krate: self.krate,
                    semantics: self.semantics,
                }
                .constructs(&self_type)
            })
            || self.reads_its_receiver(function)
            || FileLocalItem {
                definition: ModuleDef::Function(function),
                file_id: original_file_id(database, self.semantics.hir_file_for(node)),
                usages: self.usages,
            }
            .is_file_local()
    }

    fn is_exported(&self, definition: Definition<'database>) -> bool {
        let database = self.semantics.db;
        let owner = match definition {
            Definition::Function(function) => function
                .as_assoc_item(database)
                .and_then(|item| item.implementing_ty(database))
                .and_then(|self_type| self_type.as_adt()),
            Definition::Const(constant) => constant
                .as_assoc_item(database)
                .and_then(|item| item.implementing_ty(database))
                .and_then(|self_type| self_type.as_adt()),
            Definition::EnumVariant(variant) => Some(Adt::Enum(variant.parent_enum(database))),
            Definition::Field(field) => Some(match field.parent_def(database) {
                Variant::Struct(owner) => Adt::Struct(owner),
                Variant::Union(owner) => Adt::Union(owner),
                Variant::EnumVariant(owner) => Adt::Enum(owner.parent_enum(database)),
            }),
            _ => None,
        };

        definition
            .visibility(database)
            .is_some_and(|visibility| visibility.is_public())
            && definition.module(database).is_some_and(|module| {
                module.path_to_root(database).iter().all(|ancestor| {
                    ancestor.is_crate_root(database) || ancestor.visibility(database).is_public()
                })
            })
            && owner.is_none_or(|adt| adt.visibility(database).is_public())
    }

    fn is_outside_production(&self, node: &SyntaxNode) -> bool {
        self.non_production_code.contains(node) || self.in_test_gated_code(node)
    }

    fn is_own_crate_adt(&self, adt: Adt) -> bool {
        adt.module(self.semantics.db).krate(self.semantics.db) == self.krate
    }

    fn is_production_code(&self, node: &SyntaxNode) -> bool {
        !self.token_origin.is_foreign_text(node)
            && !self.non_production_code.is_in_a_non_production_file(node)
            && !Ancestry {
                node,
                semantics: self.semantics,
            }
            .enclosing()
            .is_some_and(|enclosing| self.in_test_gated_code(&enclosing))
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

    fn is_test_code(&self, node: &SyntaxNode) -> bool {
        self.non_production_code.is_test(node) || self.in_test_gated_code(node)
    }

    fn is_test_gated(&self, node: &SyntaxNode) -> bool {
        ConfiguredNode {
            node,
            options: self.production_options,
        }
        .is_disabled()
    }

    fn is_unresolved_method_call(&self, call: &ast::MethodCallExpr) -> bool {
        self.method_calls.resolve(call).is_none()
    }

    fn is_unsafe_without_the_keyword(&self, node: &SyntaxNode) -> bool {
        let database = self.semantics.db;

        (ast::Meta::cast(node.clone()).is_some_and(|meta| {
            !node
                .parent()
                .is_some_and(|parent| ast::UnsafeMeta::can_cast(parent.kind()))
                && BuiltinAttribute {
                    meta: &meta,
                    paths: self.paths,
                }
                .is_named(&UNSAFE_ATTRIBUTES)
        }) || ast::MacroCall::cast(node.clone())
            .and_then(|call| self.semantics.to_def(&call))
            .is_some_and(|expansion| {
                matches!(
                    expansion.lookup(database).def.kind,
                    MacroDefKind::BuiltIn(_, BuiltinFnLikeExpander::GlobalAsm)
                )
            }))
            && self.token_origin.is_workspace_code(node)
    }

    fn is_written_by_the_workspace(&self, node: &SyntaxNode) -> bool {
        self.token_origin.is_written_by_the_workspace(node)
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
                crate_target: self.crate_target,
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

    fn item_used_outside_production(
        &self,
        definition: Definition<'database>,
        item: UsedItem,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
        exporting_uses_checked: bool,
    ) -> Result<Option<Violation>, ModuleDependenciesError> {
        ((self
            .unpublished_packages
            .contains(&self.crate_target.package_name)
            || !self.is_exported(definition))
            && !self.is_outside_production(node))
        .then(|| {
            let references = ItemReferences {
                definition,
                item,
                node,
                semantics: self.semantics,
                usages: self.usages,
            };

            (exporting_uses_checked
                || !references.is_used_by(|user| !self.is_outside_production(user)))
            .then(|| references.using_nodes())
            .flatten()
        })
        .flatten()
        .filter(|using| {
            !using.is_empty() && using.iter().all(|user| self.is_outside_production(user))
        })
        .map(|using| {
            self.locate_focus(node, origin).map(|location| Violation {
                kind: ViolationKind::ItemUsedOnlyBy {
                    item,
                    users: if using
                        .iter()
                        .any(|user| self.non_production_code.is_benchmark(user))
                    {
                        NonProductionUsers::Benchmarks
                    } else {
                        NonProductionUsers::Tests
                    },
                },
                location,
            })
        })
        .transpose()
    }

    fn item_violations<'items>(
        &self,
        items: impl Iterator<Item = &'items ast::Item>,
        kind: &ViolationKind,
        origin: &SubtreeOrigin,
    ) -> Result<Vec<Violation>, ModuleDependenciesError> {
        items
            .filter(|item| self.is_applicable(kind.applicability(), item.syntax()))
            .map(|item| {
                self.locate_focus(item.syntax(), origin)
                    .map(|location| Violation {
                        kind: kind.clone(),
                        location,
                    })
            })
            .collect()
    }

    fn items_used_outside_production(
        &self,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
        every_configuration: &mut EveryConfigurationFindings,
    ) -> Result<Option<Violation>, ModuleDependenciesError> {
        let database = self.semantics.db;
        let used = if !matches!(origin, SubtreeOrigin::SourceFile) {
            None
        } else if let Some(written) = ast::Struct::cast(node.clone()) {
            self.semantics
                .to_def(&written)
                .map(|definition| UsedDefinition {
                    definition: Definition::Adt(Adt::Struct(definition)),
                    item: UsedItem::Struct,
                })
        } else if let Some(written) = ast::Enum::cast(node.clone()) {
            self.semantics
                .to_def(&written)
                .map(|definition| UsedDefinition {
                    definition: Definition::Adt(Adt::Enum(definition)),
                    item: UsedItem::Enum,
                })
        } else if let Some(written) = ast::Union::cast(node.clone()) {
            self.semantics
                .to_def(&written)
                .map(|definition| UsedDefinition {
                    definition: Definition::Adt(Adt::Union(definition)),
                    item: UsedItem::Union,
                })
        } else if let Some(written) = ast::Const::cast(node.clone()) {
            self.semantics
                .to_def(&written)
                .filter(|definition| {
                    definition
                        .as_assoc_item(database)
                        .is_none_or(|item| item.container_or_implemented_trait(database).is_none())
                })
                .map(|definition| UsedDefinition {
                    definition: Definition::Const(definition),
                    item: UsedItem::Constant,
                })
        } else if let Some(written) = ast::Static::cast(node.clone()) {
            self.semantics
                .to_def(&written)
                .map(|definition| UsedDefinition {
                    definition: Definition::Static(definition),
                    item: UsedItem::Static,
                })
        } else if let Some(written) = ast::TypeAlias::cast(node.clone()) {
            self.semantics
                .to_def(&written)
                .filter(|definition| {
                    definition
                        .as_assoc_item(database)
                        .is_none_or(|item| item.container_or_implemented_trait(database).is_none())
                })
                .map(|definition| UsedDefinition {
                    definition: Definition::TypeAlias(definition),
                    item: UsedItem::TypeAlias,
                })
        } else if let Some(written) = ast::Variant::cast(node.clone()) {
            self.semantics
                .to_def(&written)
                .map(|definition| UsedDefinition {
                    definition: Definition::EnumVariant(definition),
                    item: UsedItem::Variant,
                })
        } else if let Some(written) = ast::RecordField::cast(node.clone()) {
            self.semantics
                .to_def(&written)
                .map(|definition| UsedDefinition {
                    definition: Definition::Field(definition),
                    item: UsedItem::Field,
                })
        } else {
            None
        };

        used.map_or(Ok(None), |UsedDefinition { definition, item }| {
            let exporting_uses_checked = self.checks_exporting_uses(definition, item, node);

            self.export_finding(
                definition,
                item,
                node,
                origin,
                every_configuration,
                exporting_uses_checked,
            )
            .and_then(|()| {
                self.item_used_outside_production(
                    definition,
                    item,
                    node,
                    origin,
                    exporting_uses_checked,
                )
            })
        })
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

    fn loop_output_violation(
        &self,
        SubstitutedResolution {
            resolution,
            substitution,
        }: &SubstitutedResolution<'database, Function>,
    ) -> Result<Option<ViolationKind>, ModuleDependenciesError> {
        let database = self.semantics.db;

        substitution
            .as_deref()
            .and_then(|substituted| {
                substituted
                    .types(database)
                    .into_iter()
                    .find(|(parameter, _)| parameter.as_str() == BENCHMARK_LOOP_OUTPUT_PARAMETER)
            })
            .ok_or(ModuleDependenciesError::LoopOutputWithoutType(*resolution))
            .map(|(_, output)| {
                let type_name = output
                    .display(database, self.krate.to_display_target(database))
                    .to_string();

                match (BenchmarkOutput {
                    semantics: self.semantics,
                })
                .output_use(&output)
                {
                    OutputUse::Free => None,
                    OutputUse::MustUse => Some(ViolationKind::BenchmarkHarness(
                        HarnessViolation::MustUseOutput { type_name },
                    )),
                    OutputUse::Undetermined => Some(ViolationKind::BenchmarkHarness(
                        HarnessViolation::UndeterminedOutput { type_name },
                    )),
                }
            })
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
                (implementation.trait_(database).is_none() && reaches_outside).then_some(
                    ViolationKind::Placement(PlacementViolation::PublicAssociatedConstant),
                )
            }
            None => (reaches_outside && !ConstantsModule { database }.matches(module)).then_some(
                ViolationKind::Placement(PlacementViolation::PublicConstantOutsideConstantsModule),
            ),
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
                ViolationKind::Placement(PlacementViolation::MisplacedImpl(ImplPlacement::Type {
                    type_file: home_file,
                }))
            }
            LocalImplHome::TraitBlanket { .. } => ViolationKind::Placement(
                PlacementViolation::MisplacedImpl(ImplPlacement::Blanket {
                    trait_file: home_file,
                }),
            ),
            LocalImplHome::TraitObject { .. } => ViolationKind::Placement(
                PlacementViolation::MisplacedImpl(ImplPlacement::TraitObject {
                    trait_file: home_file,
                }),
            ),
            LocalImplHome::TraitForeignType { owner } => ViolationKind::Placement(
                PlacementViolation::MisplacedImpl(ImplPlacement::ForeignType {
                    trait_file: home_file,
                    trait_name: owner
                        .name(database)
                        .display(database, self.krate.edition(database))
                        .to_string(),
                }),
            ),
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
            .map(|location| {
                self.placement_rules_apply(implementation.syntax())
                    .then_some(location)
            })
            .and_then(|applicable| {
                applicable.map_or(Ok(None), |location| {
                    match (ImplementedType {
                        database,
                        implementation: definition,
                        krate: self.krate,
                    })
                    .home(&trait_arguments(implementation, self.semantics))
                    {
                        ImplHome::Foreign => Ok(Some(Violation {
                            kind: ViolationKind::Placement(
                                PlacementViolation::ImplWithoutLocalType,
                            ),
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
                .filter(|_| self.placement_rules_apply(item.syntax()))
                .map(|name| {
                    let text = name.text();

                    self.locate_focus(item.syntax(), origin)
                        .map(|location| Violation {
                            kind: ViolationKind::Placement(
                                PlacementViolation::ModuleNotNamedAfterItsItem {
                                    expected: NameWords { name: text }.snake_case(),
                                    item: text.to_owned(),
                                    module,
                                },
                            ),
                            location,
                        })
                })
                .transpose(),
            ModuleItem::Absent | ModuleItem::Accepted(_) => Ok(None),
        }
    }

    fn names_a_result_error_of_its_trait(
        &self,
        definition: TypeAlias,
    ) -> Result<bool, ModuleDependenciesError> {
        let database = self.semantics.db;

        match definition
            .as_assoc_item(database)
            .map(|item| item.container(database))
        {
            Some(AssocItemContainer::Impl(implementation)) => implementation
                .trait_(database)
                .map_or(Ok(false), |declaring| {
                    self.trait_result_errors
                        .get_or_init(declaring, || {
                            let items = declaring.items(database);

                            items
                                .iter()
                                .filter_map(|item| match item {
                                    AssocItem::Function(function) => {
                                        Some(self.result_errors_of(*function))
                                    }
                                    _ => None,
                                })
                                .collect::<Result<Vec<Vec<TypeAlias>>, ModuleDependenciesError>>()
                                .map(|errors| {
                                    let result_errors: HashSet<TypeAlias> =
                                        errors.into_iter().flatten().collect();

                                    Arc::new(
                                        items
                                            .into_iter()
                                            .filter_map(|item| match item {
                                                AssocItem::TypeAlias(declared)
                                                    if result_errors.contains(&declared) =>
                                                {
                                                    Some(declared.name(database))
                                                }
                                                _ => None,
                                            })
                                            .collect(),
                                    )
                                })
                                .map_err(Arc::new)
                        })
                        .map(|names| names.contains(&definition.name(database)))
                        .map_err(ModuleDependenciesError::TraitResultErrorsUnavailable)
                }),
            _ => Ok(false),
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
        (self.is_applicable(kind.applicability(), node)
            && self
                .token_origin
                .is_own_text(self.semantics.hir_file_for(node), node.text_range()))
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

    fn placement_rules_apply(&self, node: &SyntaxNode) -> bool {
        self.is_applicable(
            ViolationKind::Placement(PlacementViolation::AdditionalPublicItem).applicability(),
            node,
        )
    }

    fn procedural_function(
        &self,
        function: Function,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
        is_trait_item: bool,
    ) -> Result<Option<Violation>, ModuleDependenciesError> {
        let database = self.semantics.db;

        (!is_trait_item)
            .then(|| {
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
            })
            .flatten()
            .filter(|_| !self.is_exempt_from_the_procedural_rule(function, node))
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

    const fn qualified_names(&self) -> QualifiedNames<'_, 'database> {
        QualifiedNames {
            krate: self.krate,
            semantics: self.semantics,
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
                !self
                    .usages
                    .of(Definition::Local(receiver))
                    .by_file
                    .is_empty()
            })
    }

    fn record(
        &self,
        path: &ast::Path,
        origin: &SubtreeOrigin,
        ModuleFindings {
            dependencies,
            dependency_references,
            unresolved_foreign_references,
            violations,
            ..
        }: &mut ModuleFindings,
    ) -> Result<(), ModuleDependenciesError> {
        if let Some(PathResolution::Def(definition)) = self.paths.resolve(path) {
            dependency_references.extend(
                self.dependency_reference(
                    match definition {
                        ModuleDef::Module(module) => Some(module),
                        other => other.module(self.semantics.db),
                    }
                    .map(|module| module.krate(self.semantics.db)),
                    path.syntax(),
                ),
            );
        }

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

    fn record_test(&self, node: &SyntaxNode) {
        if ast::Fn::cast(node.clone()).is_some_and(|function| {
            AttributedItem {
                item: &ast::Item::Fn(function),
                paths: self.paths,
                semantics: self.semantics,
            }
            .is_libtest_test()
        }) {
            self.runs_libtest_tests.set(true);
        }
    }

    fn referenced_test(
        &self,
        function: Function,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
    ) -> Result<Option<Violation>, ModuleDependenciesError> {
        ast::Item::cast(node.clone())
            .filter(|item| {
                AttributedItem {
                    item,
                    paths: self.paths,
                    semantics: self.semantics,
                }
                .has_test_attribute()
                    && !self
                        .usages
                        .all(ModuleDef::Function(function))
                        .by_file
                        .is_empty()
            })
            .map(|_| {
                self.locate_focus(node, origin).map(|location| Violation {
                    kind: ViolationKind::ReferencedTest,
                    location,
                })
            })
            .transpose()
    }

    fn result_errors_of(
        &self,
        function: Function,
    ) -> Result<Vec<TypeAlias>, ModuleDependenciesError> {
        self.semantics
            .source(function)
            .ok_or(ModuleDependenciesError::TraitMethodWithoutSource(function))
            .map(
                |InFile {
                     value: declared, ..
                 }| {
                    declared
                        .ret_type()
                        .into_iter()
                        .flat_map(|returned| returned.syntax().descendants())
                        .chain(
                            declared
                                .param_list()
                                .into_iter()
                                .flat_map(|parameters| parameters.syntax().descendants()),
                        )
                        .filter_map(ast::PathType::cast)
                        .filter(|written| {
                            self.semantics
                                .resolve_type(&ast::Type::PathType(written.clone()))
                                .is_some_and(|resolved| {
                                    resolved.as_adt() == Some(Adt::Enum(self.standard_items.result))
                                })
                        })
                        .filter_map(|written| {
                            written
                                .path()
                                .and_then(|path| path.segment())
                                .and_then(|segment| segment.generic_arg_list())
                                .and_then(|arguments| arguments.generic_args().nth(1))
                        })
                        .filter_map(|argument| ast::TypeArg::cast(argument.syntax().clone()))
                        .filter_map(|error| error.ty())
                        .filter_map(|error| match error {
                            ast::Type::PathType(error) => error.path(),
                            _ => None,
                        })
                        .filter_map(|error| match self.semantics.resolve_path(&error) {
                            Some(PathResolution::Def(ModuleDef::TypeAlias(alias))) => Some(alias),
                            _ => None,
                        })
                        .collect()
                },
            )
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
                    .map(|tuple| {
                        self.own_violation(ViolationKind::TupleInSignature, tuple.syntax(), origin)
                    })
                    .or_else(|| {
                        ast::TupleFieldList::cast(node.clone())
                            .filter(|fields| fields.fields().nth(1).is_some())
                            .map(|fields| {
                                self.own_violation(
                                    ViolationKind::Layout(LayoutViolation::PositionalFields),
                                    fields.syntax(),
                                    origin,
                                )
                            })
                    })
                    .or_else(|| {
                        ast::Use::cast(node.clone())
                            .filter(|import| {
                                import
                                    .syntax()
                                    .parent()
                                    .is_some_and(|parent| ast::StmtList::can_cast(parent.kind()))
                            })
                            .map(|import| {
                                self.own_violation(
                                    ViolationKind::Layout(LayoutViolation::ImportInsideBlock),
                                    import.syntax(),
                                    origin,
                                )
                            })
                    })
                    .or_else(|| {
                        ast::PathType::cast(node.clone())
                            .map(|written| self.written_result_error(&written, origin))
                    })
                    .or_else(|| {
                        ast::TypeAlias::cast(node.clone())
                            .map(|alias| self.untyped_associated_error(&alias, origin))
                    })
                    .unwrap_or_else(|| self.generic_parameter_violation(node, origin))
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
        let mentions = ConfiguredNode {
            node,
            options: self.options,
        }
        .test_mentions();
        let names_test_in_a_cfg_call = ast::CfgPredExpr::cast(node.clone())
            .and_then(|call| call.cfg_predicate())
            .is_some_and(|predicate| {
                TestPredicate {
                    expression: &CfgExpr::parse_from_ast(predicate),
                }
                .names_test()
            });

        ((mentions.count > 0 || names_test_in_a_cfg_call) && self.is_production_code(node))
            .then(|| {
                let module = ast::Module::cast(node.clone());
                let kind = match &module {
                    Some(declaration) if declaration.item_list().is_none() => {
                        Some(ViolationKind::Test(TestViolation::ModuleInItsOwnFile))
                    }
                    Some(inline) if mentions.is_a_plain_gate => inline
                        .name()
                        .filter(|name| name.text() != TEST_MODULE_NAME)
                        .map(|_| ViolationKind::Test(TestViolation::ModuleNotNamedTests)),
                    _ => Some(ViolationKind::TestOnlyCodeInProduction),
                };

                kind.map(|kind| {
                    self.locate_focus(node, origin)
                        .map(|location| Violation { kind, location })
                })
                .transpose()
            })
            .transpose()
            .map(Option::flatten)
    }

    fn test_outside_a_test_module(
        &self,
        node: &SyntaxNode,
        origin: &SubtreeOrigin,
    ) -> Result<Option<Violation>, ModuleDependenciesError> {
        ast::Fn::cast(node.clone())
            .filter(|function| {
                AttributedItem {
                    item: &ast::Item::Fn(function.clone()),
                    paths: self.paths,
                    semantics: self.semantics,
                }
                .has_test_attribute()
                    && self.is_production_code(node)
            })
            .map(|_| {
                self.locate_focus(node, origin).map(|location| Violation {
                    kind: ViolationKind::Test(TestViolation::OutsideTestModule),
                    location,
                })
            })
            .transpose()
    }

    fn test_target_violations(
        &self,
        kind: &ViolationKind,
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
                    kind: kind.clone(),
                    location,
                };

                if let ItemDefinition::ModuleDef(definition) =
                    ItemDefinition::new(item, self.semantics)
                    && (FileLocalItem {
                        definition,
                        file_id: original_file_id(
                            self.semantics.db,
                            self.semantics.hir_file_for(item.syntax()),
                        ),
                        usages: self.usages,
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

    fn undeclared_environment_variable(
        &self,
        call: &ast::MacroCall,
        origin: &SubtreeOrigin,
    ) -> Result<Option<Violation>, ModuleDependenciesError> {
        let database = self.semantics.db;

        self.semantics
            .to_def(call)
            .filter(|expansion| {
                matches!(
                    expansion.lookup(database).def.kind,
                    MacroDefKind::BuiltInEager(_, expander) if expander.is_env_or_option_env()
                ) && self.token_origin.is_workspace_code(call.syntax())
            })
            .and_then(|expansion| expansion.eager_arg(database))
            .and_then(|argument| {
                code_tokens(
                    self.semantics
                        .parse_or_expand(HirFileId::from(argument))
                        .descendants_with_tokens(),
                )
                .find_map(ast::String::cast)
            })
            .map_or(Ok(None), |name| {
                name.value()
                    .map_err(
                        |error| ModuleDependenciesError::UnreadableEnvironmentVariableName {
                            literal: name.text().to_owned(),
                            reason: format!("{error:?}"),
                        },
                    )
                    .and_then(|variable| {
                        if DatabaseCrate::from(self.krate)
                            .env(database)
                            .get(&variable)
                            .is_some()
                            || self
                                .cargo_target_variables
                                .declares(self.crate_target, &variable)
                        {
                            Ok(None)
                        } else {
                            self.locate_focus(call.syntax(), origin)
                                .map(|location| Violation {
                                    kind: ViolationKind::UndeclaredEnvironmentVariable {
                                        variable: variable.into_owned(),
                                    },
                                    location,
                                })
                                .map(Some)
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
                    usages: self.usages,
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
        UnsafeCodeFindings { production, test }: &mut UnsafeCodeFindings,
    ) -> Result<(), ModuleDependenciesError> {
        code_tokens(node.children_with_tokens())
            .filter(|token| {
                token.kind() == T![unsafe]
                    && !ast::FnPtrType::can_cast(node.kind())
                    && !ast::TokenTree::can_cast(node.kind())
                    && self
                        .token_origin
                        .is_workspace_code_at(file, token.text_range())
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
            .chain(self.is_unsafe_without_the_keyword(node).then(|| {
                self.locate(node, origin).map(|location| Violation {
                    kind: ViolationKind::UnsafeCode,
                    location,
                })
            }))
            .collect::<Result<Vec<Violation>, ModuleDependenciesError>>()
            .map(|sites| {
                let integrates_foreign_code = self.integrates_foreign_code(node, file);

                if integrates_foreign_code || !sites.is_empty() {
                    if self.is_test_code(node) {
                        test
                    } else {
                        production
                    }
                    .merge(UnsafeCodeUse {
                        integrates_foreign_code,
                        sites,
                    });
                }
            })
    }

    fn untyped_associated_error(
        &self,
        alias: &ast::TypeAlias,
        origin: &SubtreeOrigin,
    ) -> Result<Option<Violation>, ModuleDependenciesError> {
        let database = self.semantics.db;

        self.semantics
            .to_def(alias)
            .map_or(Ok(None), |definition| {
                let error = definition.ty(database);

                self.names_a_result_error_of_its_trait(definition)
                    .map(|names| {
                        (names && !self.error_types().is_error_type(&error)).then_some(error)
                    })
            })
            .and_then(|untyped| {
                alias
                    .ty()
                    .zip(untyped)
                    .map_or(Ok(None), |(written, error)| {
                        self.own_violation(
                            ViolationKind::Error(self.error_boundary().untyped_violation(
                                &error,
                                UntypedErrorName::Named {
                                    error_type: written.syntax().to_string(),
                                },
                            )),
                            written.syntax(),
                            origin,
                        )
                    })
            })
    }

    fn vacuous_negations(
        &self,
        configured: &ConfiguredNode<'_>,
        origin: &SubtreeOrigin,
        ModuleFindings {
            every_configuration: EveryConfigurationFindings { held, refuted },
            ..
        }: &mut ModuleFindings,
    ) -> Result<(), ModuleDependenciesError> {
        configured
            .negated_atoms()
            .into_iter()
            .filter(|NegatedAtom { attribute, .. }| {
                !self.token_origin.is_foreign_text(attribute.syntax())
            })
            .try_for_each(
                |NegatedAtom {
                     atom,
                     attribute,
                     is_set,
                 }| {
                    self.locate(attribute.syntax(), origin).map(|location| {
                        let negation = Violation {
                            kind: ViolationKind::VacuousNegatedConfiguration {
                                atom: match atom {
                                    CfgAtom::Flag(name) => ConfigurationAtom::Flag {
                                        name: name.as_str().to_owned(),
                                    },
                                    CfgAtom::KeyValue { key, value } => {
                                        ConfigurationAtom::KeyValue {
                                            key: key.as_str().to_owned(),
                                            value: value.as_str().to_owned(),
                                        }
                                    }
                                },
                            },
                            location,
                        };

                        if is_set {
                            refuted.insert(negation);
                        } else {
                            held.insert(negation);
                        }
                    })
                },
            )
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

                if let ItemDefinition::ExternCrate(declaration) = definition {
                    findings.dependency_references.extend(
                        self.dependency_reference(
                            declaration.resolved_crate(self.semantics.db),
                            node,
                        ),
                    );
                }

                self.error_implementation(node, definition, origin, findings, crate_findings)
            })
            .and_then(|()| {
                self.error_display(node, definition, origin)
                    .map(|handwritten| findings.violations.extend(handwritten))
            })
            .and_then(|()| self.harness_call(node, origin, findings, crate_findings))
            .and_then(|()| {
                self.constructed_error(node, origin)
                    .map(|constructed| findings.violations.extend(constructed))
            })
            .and_then(|()| {
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
                    && self.is_applicable(kind.applicability(), node)
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

    fn written_result_error(
        &self,
        written: &ast::PathType,
        origin: &SubtreeOrigin,
    ) -> Result<Option<Violation>, ModuleDependenciesError> {
        WrittenResultError {
            boundary: &self.error_boundary(),
            names: &self.qualified_names(),
            paths: self.paths,
            written,
        }
        .violation()
        .and_then(|violation| {
            violation.map_or(Ok(None), |violation| {
                self.own_violation(ViolationKind::Error(violation), written.syntax(), origin)
            })
        })
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
