use std::thread::ScopedJoinHandle;

use heisenberg_project_model::configuration::Configuration;

use crate::configuration_analysis::ConfigurationAnalysis;
use crate::heisenberg_command_error::HeisenbergCommandError;

pub struct RunningAnalysis<'scope> {
    pub configuration: Configuration,
    pub handle: ScopedJoinHandle<'scope, Result<ConfigurationAnalysis, HeisenbergCommandError>>,
}
