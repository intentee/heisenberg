use std::thread::scope;

use clap::Args;
use heisenberg_analysis_threads::analysis_threads::AnalysisThreads;
use heisenberg_module_cycles::module_cycles::ModuleCycles;
use heisenberg_module_dependencies::module_dependencies::ModuleDependencies;
use heisenberg_project_model::project_model::ProjectModel;
use heisenberg_project_model::supported_target::SupportedTarget;

use crate::absolute_path::AbsolutePath;
use crate::check_report::CheckReport;
use crate::heisenberg_command_error::HeisenbergCommandError;
use crate::target_analysis::TargetAnalysis;

#[derive(Args, Debug)]
pub struct CheckCommand {
    #[arg(
        long,
        env = "CARGO_HOME",
        value_parser = AbsolutePath::parse,
        help = "Absolute path of the Cargo home directory holding the downloaded dependency sources"
    )]
    pub cargo_home: AbsolutePath,
    #[arg(
        long,
        value_parser = AbsolutePath::parse,
        help = "Absolute path of the Rust 1.99.1 sysroot that contains the rust-src component"
    )]
    pub sysroot: AbsolutePath,
    #[arg(
        long,
        value_parser = AbsolutePath::parse,
        help = "Absolute path of the Cargo workspace directory that contains Cargo.toml"
    )]
    pub workspace: AbsolutePath,
}

impl CheckCommand {
    #[expect(
        clippy::expect_used,
        reason = "a target analysis that panicked has already printed its own message, and joining re-raises it"
    )]
    pub fn execute(&self) -> Result<CheckReport, HeisenbergCommandError> {
        let project = ProjectModel {
            cargo_home: self.cargo_home.path.clone(),
            sysroot: self.sysroot.path.clone(),
            workspace_directory: self.workspace.path.clone(),
        }
        .resolve()?;
        let project = &project;
        let analyses = scope(|scope| {
            let threads = AnalysisThreads { scope };

            SupportedTarget::ALL
                .map(|target| {
                    threads.spawn(move || {
                        let loaded = project.load(target);
                        let graphs = ModuleDependencies { project: &loaded }.extract();

                        TargetAnalysis {
                            cycles: graphs
                                .iter()
                                .flat_map(|graph| ModuleCycles { graph }.find())
                                .collect(),
                            graphs,
                            target,
                        }
                    })
                })
                .map(|analysis| {
                    analysis
                        .join()
                        .expect("a target analysis that panicked has already reported why")
                })
        });

        Ok(CheckReport {
            analyses: Vec::from(analyses),
        })
    }
}
