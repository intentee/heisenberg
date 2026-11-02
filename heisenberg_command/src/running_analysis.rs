use std::thread::ScopedJoinHandle;

use heisenberg_project_model::supported_target::SupportedTarget;

use crate::heisenberg_command_error::HeisenbergCommandError;
use crate::target_analysis::TargetAnalysis;

pub struct RunningAnalysis<'scope> {
    pub handle: ScopedJoinHandle<'scope, Result<TargetAnalysis, HeisenbergCommandError>>,
    pub target: SupportedTarget,
}
