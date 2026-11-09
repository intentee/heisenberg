use std::collections::BTreeSet;
use std::collections::HashMap;

use ra_ap_cfg::CfgExpr;
use ra_ap_hir::Crate;
use ra_ap_hir::InFile;
use ra_ap_hir::Module;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::SyntaxNodePtr;
use ra_ap_vfs::FileId;

use heisenberg_module_graph::crate_target::CrateTarget;
use heisenberg_project_model::source_files::SourceFiles;
use heisenberg_task_graph::once_map::OnceMap;

use crate::module_dependencies_error::ModuleDependenciesError;
use crate::module_paths::ModulePaths;
use crate::module_walk::ModuleWalk;
use crate::path_resolutions::PathResolutions;
use crate::production_options::production_options;
use crate::source_locator::SourceLocator;
use crate::test_code::TestCode;
use crate::test_modules::TestModules;
use crate::token_origin::TokenOrigin;
use crate::walked_module::WalkedModule;

pub struct CrateWalk<'walk> {
    pub build_script_output_files: &'walk BTreeSet<FileId>,
    pub crate_target: &'walk CrateTarget,
    pub file_modules: &'walk HashMap<FileId, Vec<Module>>,
    pub files: &'walk BTreeSet<FileId>,
    pub inactive_code: &'walk HashMap<InFile<SyntaxNodePtr>, CfgExpr>,
    pub krate: Crate,
    pub module_paths: &'walk ModulePaths,
    pub source_files: &'walk SourceFiles,
    pub test_code_files: &'walk BTreeSet<FileId>,
    pub test_gated_code: &'walk OnceMap<InFile<SyntaxNodePtr>, bool>,
    pub test_modules: &'walk TestModules,
    pub unpublished_packages: &'walk BTreeSet<String>,
    pub workspace_files: &'walk BTreeSet<FileId>,
}

impl CrateWalk<'_> {
    pub fn module(
        &self,
        database: &RootDatabase,
        module: Module,
    ) -> Result<WalkedModule, ModuleDependenciesError> {
        let semantics = Semantics::new(database);
        let paths = PathResolutions::new(&semantics);
        let options = self.krate.cfg(database);
        let mut walked = WalkedModule::default();

        ModuleWalk {
            build_script_output_files: self.build_script_output_files,
            crate_target: self.crate_target,
            file_modules: self.file_modules,
            inactive_code: self.inactive_code,
            krate: self.krate,
            locator: &SourceLocator {
                database,
                files: self.source_files,
            },
            module_paths: self.module_paths,
            options,
            paths: &paths,
            production_options: &production_options(options),
            semantics: &semantics,
            test_code: &TestCode {
                files: self.test_code_files,
                modules: self.test_modules,
                semantics: &semantics,
            },
            test_gated_code: self.test_gated_code,
            token_origin: &TokenOrigin {
                own_files: self.files,
                semantics: &semantics,
                workspace_files: self.workspace_files,
            },
            unpublished_packages: self.unpublished_packages,
        }
        .module(module, &mut walked)
        .map(|()| walked)
    }
}
