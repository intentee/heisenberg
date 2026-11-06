use std::collections::BTreeSet;
use std::iter::once;

use ra_ap_hir::Crate;
use ra_ap_hir::Semantics;
use ra_ap_hir::attach_db;
use ra_ap_ide_db::base_db::Crate as DatabaseCrate;
use ra_ap_ide_db::base_db::all_crates;
use ra_ap_ide_db::base_db::salsa::Durability;
use ra_ap_ide_db::base_db::set_all_crates_with_durability;
use ra_ap_vfs::FileId;

use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_module_graph::crate_target_kind::CrateTargetKind;
use heisenberg_project_model::loaded_project::LoadedProject;

use crate::crate_extraction::CrateExtraction;
use crate::crate_module_files::crate_module_files;
use crate::member_crate::MemberCrate;
use crate::module_dependencies_error::ModuleDependenciesError;
use crate::name_resolution_priming::NameResolutionPriming;
use crate::source_locator::SourceLocator;

pub struct ModuleDependencies<'project> {
    pub project: &'project mut LoadedProject,
}

impl ModuleDependencies<'_> {
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
        .map(|()| self.member_graphs(&members))
    }

    fn crates_sharing_files(&self, members: &[MemberCrate]) -> Vec<MemberCrate> {
        let database = &self.project.database;

        attach_db(database, || {
            let semantics = Semantics::new(database);

            members
                .iter()
                .filter(|MemberCrate { krate, .. }| {
                    krate.modules(database).iter().any(|module| {
                        module
                            .as_source_file_id(database)
                            .is_some_and(|source_file_id| {
                                semantics
                                    .file_to_module_defs(source_file_id.file_id(database))
                                    .nth(1)
                                    .is_some()
                            })
                    })
                })
                .cloned()
                .collect()
        })
    }

    fn graphs(
        &self,
        members: &[MemberCrate],
        workspace_files: &BTreeSet<FileId>,
        test_code_files: &BTreeSet<FileId>,
    ) -> Vec<CrateModuleGraph> {
        let project = &*self.project;
        let database = &project.database;

        attach_db(database, || {
            let semantics = Semantics::new(database);
            let locator = SourceLocator {
                database,
                vfs: &project.vfs,
                workspace_root: &project.workspace_root,
            };

            members
                .iter()
                .map(
                    |MemberCrate {
                         crate_target,
                         krate,
                     }| {
                        CrateExtraction {
                            crate_target,
                            krate: *krate,
                            locator: &locator,
                            semantics: &semantics,
                            test_code_files,
                            unpublished_packages: &project.unpublished_packages,
                            workspace_files,
                        }
                        .graph()
                    },
                )
                .collect()
        })
    }

    fn member_graphs(&mut self, members: &[MemberCrate]) -> Vec<CrateModuleGraph> {
        let crates_sharing_files = self.crates_sharing_files(members);
        let workspace_files = self.module_files(members.iter());
        let test_code_files =
            self.module_files(members.iter().filter(|MemberCrate { crate_target, .. }| {
                crate_target.kind == CrateTargetKind::IntegrationTest
                    || self
                        .project
                        .test_crates
                        .contains(&crate_target.package_name)
            }));
        let mut graphs = self.graphs(
            &members
                .iter()
                .filter(
                    |MemberCrate {
                         krate: member_crate,
                         ..
                     }| {
                        !crates_sharing_files.iter().any(
                            |MemberCrate {
                                 krate: sharing_crate,
                                 ..
                             }| sharing_crate == member_crate,
                        )
                    },
                )
                .cloned()
                .collect::<Vec<MemberCrate>>(),
            &workspace_files,
            &test_code_files,
        );

        for member in crates_sharing_files {
            self.put_first(member.krate);
            graphs.extend(self.graphs(&[member], &workspace_files, &test_code_files));
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
                        .member_targets
                        .get(project.vfs.file_path(krate.root_file(database)))
                        .map(|crate_target| MemberCrate {
                            crate_target: crate_target.clone(),
                            krate,
                        })
                })
                .collect()
        })
    }

    fn module_files<'members>(
        &self,
        members: impl Iterator<Item = &'members MemberCrate>,
    ) -> BTreeSet<FileId> {
        let database = &self.project.database;

        attach_db(database, || {
            members
                .flat_map(|MemberCrate { krate, .. }| crate_module_files(database, *krate))
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
}
