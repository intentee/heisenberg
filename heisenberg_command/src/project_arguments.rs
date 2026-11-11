use std::collections::BTreeMap;
use std::ffi::OsString;
use std::num::NonZeroU16;

use clap::Args;

use heisenberg_absolute_path::absolute_path::AbsolutePath;
use heisenberg_project_model::configuration_selection::ConfigurationSelection;
use heisenberg_project_model::constants::CARGO_MANIFEST_FILE;
use heisenberg_project_model::project_model::ProjectModel;

use crate::constants::DEFAULT_JOBS;
use crate::constants::SUPPORTED_RUST_VERSION;

#[derive(Args, Debug)]
pub struct ProjectArguments {
    #[arg(
        long,
        env = "CARGO_HOME",
        hide_env_values = true,
        value_parser = AbsolutePath::parse,
        help = "Absolute path of the Cargo home directory holding the downloaded dependency sources"
    )]
    pub cargo_home: AbsolutePath,
    #[arg(
        long = "configuration",
        value_name = "NAME",
        required = true,
        help = "Analyse this declared configuration; repeat it to select several"
    )]
    pub configurations: Vec<String>,
    #[arg(
        long,
        default_value_t = DEFAULT_JOBS,
        help = "Number of analysis worker threads, cargo jobs and proc-macro server processes"
    )]
    pub jobs: NonZeroU16,
    #[arg(
        long,
        value_parser = AbsolutePath::parse,
        help = format!("Absolute path of the Rust {SUPPORTED_RUST_VERSION} sysroot that contains the rust-src component")
    )]
    pub sysroot: AbsolutePath,
    #[arg(
        long,
        value_parser = AbsolutePath::parse,
        help = format!("Absolute path of the Cargo workspace directory that contains {CARGO_MANIFEST_FILE}")
    )]
    pub workspace: AbsolutePath,
}

impl ProjectArguments {
    #[must_use]
    pub fn model(&self, environment: &BTreeMap<OsString, OsString>) -> ProjectModel {
        ProjectModel {
            cargo_home: self.cargo_home.path.clone(),
            configurations: ConfigurationSelection {
                names: self.configurations.clone(),
            },
            environment: environment.clone(),
            jobs: self.jobs,
            sysroot: self.sysroot.path.clone(),
            workspace_directory: self.workspace.path.clone(),
        }
    }
}
