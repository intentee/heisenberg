use std::collections::HashMap;

use cargo_metadata::Metadata;
use guppy::graph::feature::StandardFeatures;
use ra_ap_cfg::CfgAtom;
use ra_ap_cfg::CfgDiff;
use ra_ap_ide_db::FxHashMap;
use ra_ap_ide_db::base_db::Env;
use ra_ap_intern::sym;
use ra_ap_load_cargo::LoadCargoConfig;
use ra_ap_load_cargo::ProcMacroServerChoice;
use ra_ap_load_cargo::load_workspace;
use ra_ap_paths::AbsPathBuf;
use ra_ap_project_model::CargoWorkspace;
use ra_ap_project_model::CfgOverrides;
use ra_ap_project_model::ManifestPath;
use ra_ap_project_model::ProjectWorkspace;
use ra_ap_project_model::ProjectWorkspaceKind;
use ra_ap_project_model::RustSourceWorkspaceConfig;
use ra_ap_project_model::Sysroot;
use ra_ap_project_model::WorkspaceBuildScripts;
use ra_ap_vfs::VfsPath;
use serde_json::from_value;

use crate::build_selection::BuildSelection;
use crate::disabled_proc_macros::DisabledProcMacros;
use crate::exact_crate_cfg::ExactCrateCfg;
use crate::feature_graph::FeatureGraph;
use crate::library_project_json::LibraryProjectJson;
use crate::loaded_project::LoadedProject;
use crate::rustc_cfg::RustcCfg;
use crate::supported_target::SupportedTarget;

pub struct ResolvedProject {
    pub library: FeatureGraph,
    pub library_directory: AbsPathBuf,
    pub project: FeatureGraph,
    pub sysroot: AbsPathBuf,
    pub workspace_directory: AbsPathBuf,
}

impl ResolvedProject {
    #[expect(
        clippy::expect_used,
        reason = "rust-analyzer reads the metadata documents Heisenberg builds from cargo itself, returns the workspace of a sysroot with a library source root, and loading without a proc-macro server cannot fail"
    )]
    #[must_use]
    pub fn load(&self, target: SupportedTarget) -> LoadedProject {
        let crates_cfg = RustcCfg::parse(target.crates_cfg());
        let library_cfg = RustcCfg::parse(target.library_cfg());
        let platform = crates_cfg.platform(target.triple());
        let project_build = self.project.build(StandardFeatures::All, &platform);
        let library_build = self.library.build(StandardFeatures::Default, &platform);
        let project_metadata: Metadata = from_value(
            self.project
                .resolved
                .metadata(&BuildSelection::Resolved(project_build)),
        )
        .expect("cargo_metadata reads the project metadata document");
        let library_metadata: Metadata = from_value(
            self.library
                .resolved
                .metadata(&BuildSelection::Resolved(library_build)),
        )
        .expect("cargo_metadata reads the standard library metadata document");
        let mut sysroot = Sysroot::new(
            Some(self.sysroot.clone()),
            Some(self.library_directory.clone()),
        );

        let library_workspace = sysroot
            .load_workspace(
                &RustSourceWorkspaceConfig::Json(
                    LibraryProjectJson {
                        library_directory: &self.library_directory,
                        metadata: &library_metadata,
                    }
                    .project_json(),
                ),
                false,
                &drop,
            )
            .expect("rust-analyzer returns the rust-project.json workspace of a sysroot with a library source root");

        sysroot.set_workspace(library_workspace);

        let workspace = ProjectWorkspace {
            cfg_overrides: CfgOverrides {
                global: CfgDiff::new(Vec::new(), vec![CfgAtom::Flag(sym::rust_analyzer)]),
                selective: FxHashMap::default(),
            },
            extra_includes: Vec::new(),
            kind: ProjectWorkspaceKind::Cargo {
                build_scripts: WorkspaceBuildScripts::default(),
                cargo: CargoWorkspace::new(
                    project_metadata,
                    ManifestPath::try_from(self.workspace_directory.join("Cargo.toml"))
                        .expect("the workspace manifest has a parent directory"),
                    Env::default(),
                    false,
                ),
                error: None,
                rustc: Err(None),
            },
            rustc_cfg: crates_cfg.atoms,
            set_test: false,
            sysroot,
            target: Err("the target data layout is not used by name resolution".into()),
            toolchain: None,
        };
        let (mut database, vfs, _) = load_workspace(
            workspace.clone(),
            &FxHashMap::default(),
            &LoadCargoConfig {
                load_out_dirs_from_check: false,
                num_worker_threads: 1,
                prefill_caches: false,
                proc_macro_processes: 0,
                with_proc_macro_server: ProcMacroServerChoice::None,
            },
        )
        .expect("rust-analyzer loads a workspace without a proc-macro server");

        DisabledProcMacros {
            vfs: &vfs,
            workspace: &workspace,
        }
        .apply(&mut database);

        let member_targets = self
            .project
            .resolved
            .member_targets
            .iter()
            .map(|member| {
                (
                    VfsPath::from(AbsPathBuf::assert_utf8(member.root.clone())),
                    member.crate_target.clone(),
                )
            })
            .collect::<HashMap<VfsPath, _>>();

        ExactCrateCfg {
            library_atoms: &library_cfg.atoms,
            member_targets: &member_targets,
            vfs: &vfs,
        }
        .assign(&mut database);

        LoadedProject {
            database,
            member_targets,
            vfs,
            workspace_root: AbsPathBuf::assert_utf8(self.project.resolved.workspace_root.clone()),
        }
    }
}
