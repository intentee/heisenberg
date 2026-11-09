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
use ra_ap_vfs::VfsPath;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_module_graph::crate_target_kind::CrateTargetKind;
use heisenberg_project_model::loaded_project::LoadedProject;
use heisenberg_project_model::source_file::SourceFile;
use heisenberg_task_graph::once_map::OnceMap;

use crate::attached_modules::attached_modules;
use crate::crate_extraction::CrateExtraction;
use crate::crate_walk::CrateWalk;
use crate::inactive_code_site::InactiveCodeSite;
use crate::member_crate::MemberCrate;
use crate::module_dependencies_error::ModuleDependenciesError;
use crate::module_diagnostics::ModuleDiagnostics;
use crate::module_paths::ModulePaths;
use crate::name_resolution_priming::NameResolutionPriming;
use crate::resolved_member_crate::ResolvedMemberCrate;
use crate::source_locator::SourceLocator;
use crate::test_modules::TestModules;

fn inactive_code(diagnostics: &[ModuleDiagnostics]) -> HashMap<InFile<SyntaxNodePtr>, CfgExpr> {
    diagnostics
        .iter()
        .flat_map(|ModuleDiagnostics { inactive_code, .. }| inactive_code)
        .map(|InactiveCodeSite { cfg, node }| (*node, cfg.clone()))
        .collect()
}

fn module_files<'members>(
    members: impl Iterator<Item = &'members ResolvedMemberCrate>,
) -> BTreeSet<FileId> {
    members
        .flat_map(|ResolvedMemberCrate { files, .. }| files.iter().copied())
        .collect()
}

pub struct ModuleDependencies<'project, 'resolved> {
    pub project: &'project mut LoadedProject<'resolved>,
}

impl ModuleDependencies<'_, '_> {
    /// # Errors
    ///
    /// Returns [`ModuleDependenciesError`] when name resolution cannot be scheduled over the
    /// crate dependency graph, or when rust-analyzer refers to a source file or offset that the
    /// loaded project does not hold.
    pub fn extract(&mut self) -> Result<Vec<CrateModuleGraph>, ModuleDependenciesError> {
        self.members().and_then(|members| {
            NameResolutionPriming {
                database: &self.project.database,
                members: &members
                    .iter()
                    .map(|MemberCrate { krate, .. }| *krate)
                    .collect::<Vec<Crate>>(),
            }
            .prime()
            .and_then(|()| {
                let resolved = self.resolved_members(members);

                self.member_graphs(&resolved)
            })
        })
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
                    let diagnostics = attached_modules(
                        database,
                        &modules_with_files,
                        |module_database, module| {
                            ModuleDiagnostics::new(&Semantics::new(module_database), module)
                        },
                    );
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
                    let walked = attached_modules(
                        database,
                        &modules_with_files,
                        |module_database, module| crate_walk.module(module_database, module),
                    );

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
                crate_target.kind == CrateTargetKind::IntegrationTest
                    || self
                        .project
                        .packages
                        .test_crates
                        .contains(&crate_target.package_name)
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

    fn members(&self) -> Result<Vec<MemberCrate>, ModuleDependenciesError> {
        let project = &*self.project;
        let database = &project.database;

        attach_db(database, || {
            Crate::all(database)
                .into_iter()
                .filter_map(|krate| {
                    let root = krate.root_file(database);

                    project
                        .files
                        .file(root)
                        .ok_or(ModuleDependenciesError::UnknownSourceFile(root))
                        .map(|SourceFile { path, .. }| {
                            project
                                .packages
                                .member_targets
                                .get(&VfsPath::from(path.to_path_buf()))
                                .map(|crate_target| MemberCrate {
                                    crate_target: crate_target.clone(),
                                    krate,
                                })
                        })
                        .transpose()
                })
                .collect()
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

    fn resolved_members(&self, members: Vec<MemberCrate>) -> Vec<ResolvedMemberCrate> {
        let database = &self.project.database;

        attach_db(database, || {
            members
                .into_iter()
                .map(|member| member.resolved(database))
                .collect()
        })
    }
}
