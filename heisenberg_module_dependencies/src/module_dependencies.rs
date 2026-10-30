use std::iter::once;

use ra_ap_hir::Crate;
use ra_ap_hir::Semantics;
use ra_ap_hir::attach_db;
use ra_ap_ide_db::base_db::Crate as DatabaseCrate;
use ra_ap_ide_db::base_db::all_crates;
use ra_ap_ide_db::base_db::salsa::Durability;
use ra_ap_ide_db::base_db::set_all_crates_with_durability;

use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_project_model::loaded_project::LoadedProject;

use crate::crate_extraction::CrateExtraction;
use crate::name_resolution_priming::NameResolutionPriming;
use crate::source_locator::SourceLocator;

pub struct ModuleDependencies<'project> {
    pub project: &'project mut LoadedProject,
}

impl ModuleDependencies<'_> {
    #[must_use]
    pub fn extract(&mut self) -> Vec<CrateModuleGraph> {
        let members = self.members();

        NameResolutionPriming {
            database: &self.project.database,
            members: &members,
        }
        .prime();

        let crates_sharing_files = self.crates_sharing_files(&members);
        let mut graphs = self.graphs(
            &members
                .iter()
                .copied()
                .filter(|krate| !crates_sharing_files.contains(krate))
                .collect::<Vec<Crate>>(),
        );

        for krate in crates_sharing_files {
            self.put_first(krate);
            graphs.extend(self.graphs(&[krate]));
        }

        graphs.sort();

        graphs
    }

    fn crates_sharing_files(&self, members: &[Crate]) -> Vec<Crate> {
        let database = &self.project.database;

        attach_db(database, || {
            let semantics = Semantics::new(database);

            members
                .iter()
                .copied()
                .filter(|krate| {
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
                .collect()
        })
    }

    fn graphs(&self, crates: &[Crate]) -> Vec<CrateModuleGraph> {
        let project = &*self.project;
        let database = &project.database;

        attach_db(database, || {
            let semantics = Semantics::new(database);
            let locator = SourceLocator {
                database,
                vfs: &project.vfs,
                workspace_root: &project.workspace_root,
            };

            crates
                .iter()
                .filter_map(|krate| {
                    project
                        .member_targets
                        .get(project.vfs.file_path(krate.root_file(database)))
                        .map(|crate_target| {
                            CrateExtraction {
                                crate_target,
                                krate: *krate,
                                locator: &locator,
                                semantics: &semantics,
                            }
                            .graph()
                        })
                })
                .collect()
        })
    }

    fn members(&self) -> Vec<Crate> {
        let project = &*self.project;
        let database = &project.database;

        attach_db(database, || {
            Crate::all(database)
                .into_iter()
                .filter(|krate| {
                    project
                        .member_targets
                        .contains_key(project.vfs.file_path(krate.root_file(database)))
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
}
