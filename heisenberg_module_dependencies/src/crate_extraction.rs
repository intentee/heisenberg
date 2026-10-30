use std::collections::BTreeSet;

use ra_ap_hir::Crate;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_vfs::FileId;

use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_module_graph::crate_target::CrateTarget;

use crate::diagnostic_violation::DiagnosticViolation;
use crate::module_paths::ModulePaths;
use crate::module_walk::ModuleWalk;
use crate::source_locator::SourceLocator;
use crate::token_origin::TokenOrigin;

pub struct CrateExtraction<'extraction, 'database> {
    pub crate_target: &'extraction CrateTarget,
    pub krate: Crate,
    pub locator: &'extraction SourceLocator<'extraction>,
    pub semantics: &'extraction Semantics<'database, RootDatabase>,
}

impl CrateExtraction<'_, '_> {
    pub fn graph(&self) -> CrateModuleGraph {
        let database = self.semantics.db;
        let module_paths = ModulePaths { database };
        let modules = self.krate.modules(database);
        let own_files: BTreeSet<FileId> = modules
            .iter()
            .filter_map(|module| module.as_source_file_id(database))
            .map(|source_file_id| source_file_id.file_id(database))
            .collect();
        let token_origin = TokenOrigin {
            own_files: &own_files,
            semantics: self.semantics,
        };
        let walk = ModuleWalk {
            crate_target_kind: self.crate_target.kind,
            krate: self.krate,
            locator: self.locator,
            module_paths: &module_paths,
            options: self.krate.cfg(database),
            semantics: self.semantics,
            token_origin: &token_origin,
        };
        let mut graph = CrateModuleGraph {
            crate_target: self.crate_target.clone(),
            dependencies: BTreeSet::new(),
            inactive_regions: BTreeSet::new(),
            modules: modules
                .iter()
                .map(|module| module_paths.path(*module))
                .collect(),
            shared_module_files: BTreeSet::new(),
            violations: BTreeSet::new(),
            walked_files: BTreeSet::new(),
        };

        for module in &modules {
            walk.module(*module, &mut graph);

            let mut diagnostics = Vec::new();

            module.diagnostics(database, &mut diagnostics, false);
            graph
                .violations
                .extend(diagnostics.iter().filter_map(|diagnostic| {
                    DiagnosticViolation {
                        diagnostic,
                        locator: self.locator,
                        semantics: self.semantics,
                        token_origin: &token_origin,
                    }
                    .violation()
                }));
        }

        graph
    }
}
