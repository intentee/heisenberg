use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use ra_ap_cfg::CfgExpr;
use ra_ap_hir::Crate;
use ra_ap_hir::InFile;
use ra_ap_hir::Module;
use ra_ap_hir::Semantics;
use ra_ap_hir::Trait;
use ra_ap_hir_expand::name::Name;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::SyntaxNodePtr;
use ra_ap_vfs::FileId;

use heisenberg_configured_code::non_production_code::NonProductionCode;
use heisenberg_configured_code::non_production_files::NonProductionFiles;
use heisenberg_configured_code::test_module_code::TestModuleCode;
use heisenberg_configured_code::test_modules::TestModules;
use heisenberg_module_graph::crate_target::CrateTarget;
use heisenberg_project_model::source_files::SourceFiles;
use heisenberg_source_locator::source_locator::SourceLocator;
use heisenberg_task_graph::once_map::OnceMap;

use crate::benchmark_harness::BenchmarkHarness;
use crate::definition_usages::DefinitionUsages;
use crate::method_call_resolutions::MethodCallResolutions;
use crate::module_dependencies_error::ModuleDependenciesError;
use crate::module_paths::ModulePaths;
use crate::module_walk::ModuleWalk;
use crate::path_resolutions::PathResolutions;
use crate::serialization_traits::SerializationTraits;
use crate::standard_items::StandardItems;
use crate::template_captures::TemplateCaptures;
use crate::test_assertions::TestAssertions;
use crate::token_origin::TokenOrigin;
use crate::vetted_crates::VettedCrates;
use crate::walked_module::WalkedModule;

pub struct CrateWalk<'walk> {
    pub build_script_output_files: &'walk BTreeSet<FileId>,
    pub crate_target: &'walk CrateTarget,
    pub file_modules: &'walk HashMap<FileId, Vec<Module>>,
    pub files: &'walk BTreeSet<FileId>,
    pub inactive_code: &'walk HashMap<InFile<SyntaxNodePtr>, CfgExpr>,
    pub krate: Crate,
    pub module_paths: &'walk ModulePaths,
    pub non_production_files: &'walk NonProductionFiles,
    pub serialization_traits: &'walk SerializationTraits,
    pub source_files: &'walk SourceFiles,
    pub standard_items: &'walk StandardItems,
    pub test_gated_code: &'walk OnceMap<InFile<SyntaxNodePtr>, bool>,
    pub test_modules: &'walk TestModules,
    pub trait_result_errors: &'walk OnceMap<Trait, Arc<HashSet<Name>>>,
    pub unpublished_packages: &'walk BTreeSet<String>,
    pub vetted_crates: &'walk VettedCrates,
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
        let method_calls = MethodCallResolutions::new(&semantics);
        let options = self.krate.cfg(database);
        let production_options = self.test_modules.production_options(database, self.krate);
        let template_captures = TemplateCaptures::new(&semantics);
        let usages = DefinitionUsages::new(&semantics);
        let test_assertions = TestAssertions::default();
        let mut walked = WalkedModule::default();

        ModuleWalk {
            benchmark_harness: &BenchmarkHarness {
                crates: &self.vetted_crates.benchmark_harness,
                semantics: &semantics,
            },
            build_script_output_files: self.build_script_output_files,
            crate_target: self.crate_target,
            file_modules: self.file_modules,
            inactive_code: self.inactive_code,
            krate: self.krate,
            locator: &SourceLocator {
                database,
                files: self.source_files,
            },
            method_calls: &method_calls,
            module_paths: self.module_paths,
            non_production_code: &NonProductionCode {
                files: self.non_production_files,
                modules: TestModuleCode {
                    modules: self.test_modules,
                    semantics: &semantics,
                },
            },
            options,
            paths: &paths,
            production_options: &production_options,
            semantics: &semantics,
            serialization_traits: self.serialization_traits,
            template_captures: &template_captures,
            test_gated_code: self.test_gated_code,
            trait_result_errors: self.trait_result_errors,
            token_origin: &TokenOrigin {
                own_files: self.files,
                semantics: &semantics,
                workspace_files: self.workspace_files,
            },
            standard_items: self.standard_items,
            unpublished_packages: self.unpublished_packages,
            test_assertions: &test_assertions,
            error_derive_crates: &self.vetted_crates.error_derive,
            error_runtime_crates: &self.vetted_crates.error_runtime,
            usages: &usages,
        }
        .module(module, &mut walked)
        .map(|()| walked)
    }
}
