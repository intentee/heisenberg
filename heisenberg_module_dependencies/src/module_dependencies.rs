use std::collections::BTreeSet;
use std::collections::HashMap;
use std::iter::once;

use ra_ap_cfg::CfgExpr;
use ra_ap_hir::Crate;
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
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use heisenberg_attached_work::attached_work::attached_work;
use heisenberg_configured_code::test_modules::TestModules;
use heisenberg_member_crates::member_crates::member_crates;
use heisenberg_member_crates::module_files::module_files;
use heisenberg_member_crates::resolved_member_crate::ResolvedMemberCrate;
use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_project_model::loaded_project::LoadedProject;
use heisenberg_source_locator::source_locator::SourceLocator;
use heisenberg_task_graph::once_map::OnceMap;

use crate::crate_extraction::CrateExtraction;
use crate::crate_walk::CrateWalk;
use crate::inactive_code_site::InactiveCodeSite;
use crate::module_dependencies_error::ModuleDependenciesError;
use crate::module_diagnostics::ModuleDiagnostics;
use crate::module_paths::ModulePaths;

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
            .and_then(|resolved| self.member_graphs(&resolved))
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
        members: &[ResolvedMemberCrate],
        workspace_files: &BTreeSet<FileId>,
        test_code_files: &BTreeSet<FileId>,
        file_modules: &HashMap<FileId, Vec<Module>>,
    ) -> Result<Vec<CrateModuleGraph>, ModuleDependenciesError> {
        let project = &*self.project;
        let build_script_output_files = &project.build_script_output_files;
        let files = &project.files;
        let unpublished_packages = &project.packages.unpublished_packages;
        let module_paths = ModulePaths::default();
        let test_gated_code = OnceMap::default();
        let test_modules = TestModules::default();

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
                    let diagnostics =
                        attached_work(database, &modules_with_files, |module_database, module| {
                            ModuleDiagnostics::new(&Semantics::new(module_database), *module)
                        });
                    let inactive_code = inactive_code(&diagnostics);
                    let crate_walk = CrateWalk {
                        build_script_output_files,
                        crate_target,
                        file_modules,
                        files: member_files,
                        inactive_code: &inactive_code,
                        krate: *krate,
                        module_paths: &module_paths,
                        source_files: files,
                        test_code_files,
                        test_gated_code: &test_gated_code,
                        test_modules: &test_modules,
                        unpublished_packages,
                        workspace_files,
                    };
                    let walked =
                        attached_work(database, &modules_with_files, |module_database, module| {
                            crate_walk.module(module_database, *module)
                        });

                    attach_db(database, || {
                        CrateExtraction {
                            crate_target,
                            diagnostics: &diagnostics,
                            files: member_files,
                            krate: *krate,
                            locator: &SourceLocator { database, files },
                            module_paths: &module_paths,
                            modules: &modules,
                            semantics: &Semantics::new(database),
                            workspace_files,
                        }
                        .graph(walked)
                    })
                },
            )
            .collect::<Vec<Result<CrateModuleGraph, ModuleDependenciesError>>>()
            .into_iter()
            .collect()
    }

    fn member_graphs(
        &mut self,
        members: &[ResolvedMemberCrate],
    ) -> Result<Vec<CrateModuleGraph>, ModuleDependenciesError> {
        let workspace_files = module_files(members.iter());
        let test_code_files = module_files(members.iter().filter(
            |ResolvedMemberCrate { crate_target, .. }| {
                crate_target.is_test_code(&self.project.packages.test_crates)
            },
        ));
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
            &workspace_files,
            &test_code_files,
            &file_modules,
        )
        .and_then(|mut graphs| {
            crates_sharing_files
                .into_iter()
                .try_for_each(|member| {
                    self.put_first(member.krate);

                    let shared_file_modules = self.file_modules(&member.files);

                    self.graphs(
                        &[member],
                        &workspace_files,
                        &test_code_files,
                        &shared_file_modules,
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
}
