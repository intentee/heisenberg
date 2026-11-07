use std::cell::RefCell;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::iter::once;

use ra_ap_hir::Crate;
use ra_ap_hir::Module;
use ra_ap_hir::Semantics;
use ra_ap_hir::attach_db;
use ra_ap_ide_db::base_db::Crate as DatabaseCrate;
use ra_ap_ide_db::base_db::all_crates;
use ra_ap_ide_db::base_db::salsa::Durability;
use ra_ap_ide_db::base_db::set_all_crates_with_durability;
use ra_ap_vfs::FileId;
use ra_ap_vfs::VfsPath;

use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_module_graph::crate_target_kind::CrateTargetKind;
use heisenberg_project_model::loaded_project::LoadedProject;

use crate::crate_extraction::CrateExtraction;
use crate::member_crate::MemberCrate;
use crate::module_dependencies_error::ModuleDependenciesError;
use crate::name_resolution_priming::NameResolutionPriming;
use crate::resolved_member_crate::ResolvedMemberCrate;
use crate::source_locator::SourceLocator;

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
    /// Returns [`ModuleDependenciesError`] when the host does not report its available
    /// parallelism, or when a name resolution thread cannot be started or panics.
    pub fn extract(&mut self) -> Result<Vec<CrateModuleGraph>, ModuleDependenciesError> {
        let members = self.members();

        NameResolutionPriming {
            database: &self.project.database,
            members: &members
                .iter()
                .map(|MemberCrate { krate, .. }| *krate)
                .collect::<Vec<Crate>>(),
        }
        .prime()
        .map(|()| {
            let resolved = self.resolved_members(members);

            self.member_graphs(&resolved)
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
        relative_paths: &RefCell<HashMap<FileId, String>>,
    ) -> Vec<CrateModuleGraph> {
        let project = &*self.project;
        let database = &project.database;

        attach_db(database, || {
            let semantics = Semantics::new(database);
            let locator = SourceLocator {
                database,
                files: &project.files,
                relative_paths,
                workspace_root: project.workspace_root,
            };

            members
                .iter()
                .map(
                    |ResolvedMemberCrate {
                         crate_target,
                         files,
                         krate,
                     }| {
                        CrateExtraction {
                            build_script_output_files: &project.build_script_output_files,
                            crate_target,
                            file_modules,
                            files,
                            krate: *krate,
                            locator: &locator,
                            semantics: &semantics,
                            test_code_files,
                            unpublished_packages: &project.packages.unpublished_packages,
                            workspace_files,
                        }
                        .graph()
                    },
                )
                .collect()
        })
    }

    fn member_graphs(&mut self, members: &[ResolvedMemberCrate]) -> Vec<CrateModuleGraph> {
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
        let relative_paths = RefCell::default();
        let crates_sharing_files: Vec<ResolvedMemberCrate> = members
            .iter()
            .filter(|ResolvedMemberCrate { files, .. }| {
                files.iter().any(|file| file_modules[file].len() > 1)
            })
            .cloned()
            .collect();
        let mut graphs = self.graphs(
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
            &relative_paths,
        );

        for member in crates_sharing_files {
            self.put_first(member.krate);

            let shared_file_modules = self.file_modules(&member.files);

            graphs.extend(self.graphs(
                &[member],
                &workspace_files,
                &test_code_files,
                &shared_file_modules,
                &relative_paths,
            ));
        }

        graphs.sort();

        graphs
    }

    fn members(&self) -> Vec<MemberCrate> {
        let project = &*self.project;
        let database = &project.database;

        attach_db(database, || {
            Crate::all(database)
                .into_iter()
                .filter_map(|krate| {
                    project
                        .packages
                        .member_targets
                        .get(&VfsPath::from(
                            project.files.path(krate.root_file(database)).to_path_buf(),
                        ))
                        .map(|crate_target| MemberCrate {
                            crate_target: crate_target.clone(),
                            krate,
                        })
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
