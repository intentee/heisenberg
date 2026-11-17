use std::collections::BTreeSet;
use std::collections::HashMap;
use std::iter::once;

use ra_ap_cfg::CfgExpr;
use ra_ap_hir::Crate;
use ra_ap_hir::CrateDependency;
use ra_ap_hir::InFile;
use ra_ap_hir::Module;
use ra_ap_hir::Semantics;
use ra_ap_hir::attach_db;
use ra_ap_ide_db::base_db::Crate as DatabaseCrate;
use ra_ap_ide_db::base_db::all_crates;
use ra_ap_ide_db::base_db::salsa::Durability;
use ra_ap_ide_db::base_db::set_all_crates_with_durability;
use ra_ap_syntax::SyntaxNodePtr;
use ra_ap_vfs::FileId;
use ra_ap_vfs::VfsPath;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use heisenberg_attached_work::attached_work::attached_work;
use heisenberg_configured_code::non_production_files::NonProductionFiles;
use heisenberg_configured_code::test_modules::TestModules;
use heisenberg_member_crates::member_crates::member_crates;
use heisenberg_member_crates::module_files::module_files;
use heisenberg_member_crates::resolved_member_crate::ResolvedMemberCrate;
use heisenberg_module_graph::code_category::CodeCategory;
use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_project_model::loaded_project::LoadedProject;
use heisenberg_project_model::source_file::SourceFile;
use heisenberg_source_locator::source_locator::SourceLocator;
use heisenberg_task_graph::once_map::OnceMap;

use crate::cargo_target_variables::CargoTargetVariables;
use crate::crate_extraction::CrateExtraction;
use crate::crate_walk::CrateWalk;
use crate::inactive_code_site::InactiveCodeSite;
use crate::module_dependencies_error::ModuleDependenciesError;
use crate::module_diagnostics::ModuleDiagnostics;
use crate::module_paths::ModulePaths;
use crate::searched_tokens::SearchedTokens;
use crate::standard_items::StandardItems;
use crate::vetted_crates::VettedCrates;

fn inactive_code(diagnostics: &[ModuleDiagnostics]) -> HashMap<InFile<SyntaxNodePtr>, CfgExpr> {
    diagnostics
        .iter()
        .flat_map(|ModuleDiagnostics { inactive_code, .. }| inactive_code)
        .map(|InactiveCodeSite { cfg, node }| (*node, cfg.clone()))
        .collect()
}

pub struct ModuleDependencies<'project, 'resolved> {
    pub project: &'project mut LoadedProject<'resolved>,
}

impl ModuleDependencies<'_, '_> {
    /// # Errors
    ///
    /// Returns [`ModuleDependenciesError`] when name resolution and trait implementations
    /// cannot be scheduled over the crate dependency graph, or when rust-analyzer refers to a source file or offset that the
    /// loaded project does not hold.
    pub fn extract(&mut self) -> Result<Vec<CrateModuleGraph>, ModuleDependenciesError> {
        member_crates(self.project)
            .map_err(ModuleDependenciesError::MemberCrates)
            .and_then(|resolved| {
                self.vetted_crates()
                    .and_then(|vetted_crates| self.member_graphs(&vetted_crates, &resolved))
            })
    }

    fn category_files(
        &self,
        members: &[ResolvedMemberCrate],
        category: CodeCategory,
    ) -> BTreeSet<FileId> {
        module_files(
            members
                .iter()
                .filter(|ResolvedMemberCrate { crate_target, .. }| {
                    crate_target.code_category(&self.project.packages.test_crates) == category
                }),
        )
    }

    fn file_modules(&self, files: &BTreeSet<FileId>) -> HashMap<FileId, Vec<Module>> {
        let database = &self.project.database;

        attach_db(database, || {
            let semantics = Semantics::new(database);

            files
                .iter()
                .map(|file| (*file, semantics.file_to_module_defs(*file).collect()))
                .collect()
        })
    }

    fn graphs(
        &self,
        vetted_crates: &VettedCrates,
        members: &[ResolvedMemberCrate],
        cargo_target_variables: &CargoTargetVariables,
        workspace_files: &BTreeSet<FileId>,
        non_production_files: &NonProductionFiles,
        file_modules: &HashMap<FileId, Vec<Module>>,
    ) -> Result<Vec<CrateModuleGraph>, ModuleDependenciesError> {
        let project = &*self.project;
        let build_script_output_files = &project.build_script_output_files;
        let files = &project.files;
        let unpublished_packages = &project.packages.unpublished_packages;
        let module_paths = ModulePaths::default();
        let test_gated_code = OnceMap::default();
        let test_modules = TestModules::default();
        let trait_result_errors = OnceMap::default();
        let searched_tokens = attach_db(&project.database, || {
            SearchedTokens::new(&Semantics::new(&project.database), workspace_files)
        });

        members
            .par_iter()
            .map_with(
                project.database.clone(),
                |database,
                 ResolvedMemberCrate {
                     crate_target,
                     files: member_files,
                     krate,
                 }| {
                    let database = &*database;
                    let modules = attach_db(database, || krate.modules(database));
                    let modules_with_files: Vec<Module> = attach_db(database, || {
                        modules
                            .iter()
                            .filter(|module| !module.is_inline(database))
                            .copied()
                            .collect()
                    });
                    attach_db(database, || {
                        StandardItems::new(&Semantics::new(database), *krate, crate_target)
                    })
                    .and_then(|standard_items| {
                        let diagnostics = attached_work(
                            database,
                            &modules_with_files,
                            |module_database, module| {
                                ModuleDiagnostics::new(&Semantics::new(module_database), *module)
                            },
                        );
                        let inactive_code = inactive_code(&diagnostics);
                        let dependency_names: HashMap<Crate, String> = attach_db(database, || {
                            krate
                                .dependencies(database)
                                .into_iter()
                                .map(|CrateDependency { krate, name }| {
                                    (krate, name.as_str().to_owned())
                                })
                                .collect()
                        });
                        let crate_walk = CrateWalk {
                            build_script_output_files,
                            cargo_target_variables,
                            crate_target,
                            dependency_names: &dependency_names,
                            file_modules,
                            files: member_files,
                            inactive_code: &inactive_code,
                            krate: *krate,
                            module_paths: &module_paths,
                            non_production_files,
                            searched_tokens: &searched_tokens,
                            source_files: files,
                            standard_items: &standard_items,
                            test_gated_code: &test_gated_code,
                            test_modules: &test_modules,
                            trait_result_errors: &trait_result_errors,
                            unpublished_packages,
                            vetted_crates,
                            workspace_files,
                        };
                        let walked = attached_work(
                            database,
                            &modules_with_files,
                            |module_database, module| crate_walk.module(module_database, *module),
                        );

                        attach_db(database, || {
                            CrateExtraction {
                                crate_target,
                                diagnostics: &diagnostics,
                                files: member_files,
                                krate: *krate,
                                locator: &SourceLocator { database, files },
                                modules: &modules,
                                semantics: &Semantics::new(database),
                            }
                            .graph(walked)
                        })
                    })
                },
            )
            .collect::<Vec<Result<CrateModuleGraph, ModuleDependenciesError>>>()
            .into_iter()
            .collect()
    }

    fn member_graphs(
        &mut self,
        vetted_crates: &VettedCrates,
        members: &[ResolvedMemberCrate],
    ) -> Result<Vec<CrateModuleGraph>, ModuleDependenciesError> {
        let workspace_files = module_files(members.iter());
        let cargo_target_variables = CargoTargetVariables::new(
            members
                .iter()
                .map(|ResolvedMemberCrate { crate_target, .. }| crate_target),
        );
        let non_production_files = NonProductionFiles {
            benchmark: self.category_files(members, CodeCategory::Benchmark),
            test: self.category_files(members, CodeCategory::Test),
        };
        let file_modules = self.file_modules(&workspace_files);
        let shared_files: BTreeSet<FileId> = file_modules
            .iter()
            .filter(|(_, modules)| modules.len() > 1)
            .map(|(file, _)| *file)
            .collect();
        let crates_sharing_files: Vec<ResolvedMemberCrate> = members
            .iter()
            .filter(|ResolvedMemberCrate { files, .. }| {
                files.iter().any(|file| shared_files.contains(file))
            })
            .cloned()
            .collect();
        self.graphs(
            vetted_crates,
            &members
                .iter()
                .filter(
                    |ResolvedMemberCrate {
                         krate: member_crate,
                         ..
                     }| {
                        !crates_sharing_files.iter().any(
                            |ResolvedMemberCrate {
                                 krate: sharing_crate,
                                 ..
                             }| sharing_crate == member_crate,
                        )
                    },
                )
                .cloned()
                .collect::<Vec<ResolvedMemberCrate>>(),
            &cargo_target_variables,
            &workspace_files,
            &non_production_files,
            &file_modules,
        )
        .and_then(|mut graphs| {
            crates_sharing_files
                .into_iter()
                .try_for_each(|member| {
                    self.put_first(member.krate);

                    let member_file_modules: HashMap<FileId, Vec<Module>> = file_modules
                        .iter()
                        .filter(|(file, _)| member.files.contains(*file))
                        .map(|(file, modules)| (*file, modules.clone()))
                        .chain(self.file_modules(
                            &member.files.intersection(&shared_files).copied().collect(),
                        ))
                        .collect();

                    self.graphs(
                        vetted_crates,
                        &[member],
                        &cargo_target_variables,
                        &workspace_files,
                        &non_production_files,
                        &member_file_modules,
                    )
                    .map(|shared| graphs.extend(shared))
                })
                .map(|()| {
                    graphs.sort();

                    graphs
                })
        })
    }

    fn put_first(&mut self, krate: Crate) {
        let database = &mut self.project.database;
        let first = DatabaseCrate::from(krate);
        let order: Vec<DatabaseCrate> = once(first)
            .chain(
                all_crates(database)
                    .iter()
                    .copied()
                    .filter(|other| *other != first),
            )
            .collect();

        set_all_crates_with_durability(database, order, Durability::MEDIUM);
    }

    fn vetted_crates(&self) -> Result<VettedCrates, ModuleDependenciesError> {
        let LoadedProject {
            database,
            files,
            packages,
            ..
        } = &*self.project;
        let locator = SourceLocator { database, files };

        attach_db(database, || {
            Crate::all(database).into_iter().try_fold(
                VettedCrates::default(),
                |mut vetted, krate| {
                    locator
                        .source_file::<ModuleDependenciesError>(krate.root_file(database))
                        .map(|SourceFile { path, .. }| {
                            let root = VfsPath::from(path.to_path_buf());

                            if packages.benchmark_harness_roots.contains(&root) {
                                vetted.benchmark_harness.insert(krate);
                            }

                            if packages.error_derive_roots.contains(&root) {
                                vetted.error_derive.insert(krate);
                            }

                            if packages.error_runtime_roots.contains(&root) {
                                vetted.error_runtime.insert(krate);
                            }

                            vetted
                        })
                },
            )
        })
    }
}
