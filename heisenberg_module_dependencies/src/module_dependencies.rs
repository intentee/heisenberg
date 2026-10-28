use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_project_model::loaded_project::LoadedProject;
use ra_ap_hir::Crate;
use ra_ap_hir::Semantics;
use ra_ap_hir::attach_db;

use crate::crate_extraction::CrateExtraction;
use crate::name_resolution_priming::NameResolutionPriming;
use crate::source_locator::SourceLocator;

pub struct ModuleDependencies<'project> {
    pub project: &'project LoadedProject,
}

impl ModuleDependencies<'_> {
    #[must_use]
    pub fn extract(&self) -> Vec<CrateModuleGraph> {
        let project = self.project;
        let database = &project.database;

        attach_db(database, || {
            let semantics = Semantics::new(database);
            let locator = SourceLocator {
                database,
                vfs: &project.vfs,
                workspace_root: &project.workspace_root,
            };
            let extractions: Vec<CrateExtraction<'_, '_>> = Crate::all(database)
                .into_iter()
                .filter_map(|krate| {
                    project
                        .member_targets
                        .get(project.vfs.file_path(krate.root_file(database)))
                        .map(|crate_target| CrateExtraction {
                            crate_target,
                            krate,
                            locator: &locator,
                            semantics: &semantics,
                        })
                })
                .collect();
            let members: Vec<Crate> = extractions
                .iter()
                .map(|extraction| extraction.krate)
                .collect();

            NameResolutionPriming {
                database,
                members: &members,
            }
            .prime();

            let mut graphs: Vec<CrateModuleGraph> =
                extractions.iter().map(CrateExtraction::graph).collect();

            graphs.sort();

            graphs
        })
    }
}
