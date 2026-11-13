use std::path::Path;

use cargo::GlobalContext;
use cargo::workspace::Workspace;
use cargo_util_terminal::Shell;

use crate::captured_output::CapturedOutput;
use crate::configured_context::ConfiguredContext;
use crate::workspace_build_error::WorkspaceBuildError;

pub struct CargoSession {
    pub(crate) captured: CapturedOutput,
    pub(crate) context: GlobalContext,
}

impl CargoSession {
    /// # Errors
    ///
    /// Returns [`WorkspaceBuildError`] when the cargo configuration of the build cannot be
    /// read.
    pub fn new(context: &ConfiguredContext<'_>) -> Result<Self, WorkspaceBuildError> {
        let captured = CapturedOutput::default();

        context
            .with_shell(Shell::from_write(Box::new(captured.clone())))
            .map_err(Box::new)
            .map_err(WorkspaceBuildError::CargoConfiguration)
            .map(|context| Self { captured, context })
    }

    /// # Errors
    ///
    /// Returns [`WorkspaceBuildError`] when cargo cannot load the workspace of
    /// `manifest_path`.
    pub fn workspace(&self, manifest_path: &Path) -> Result<Workspace<'_>, WorkspaceBuildError> {
        Workspace::new(manifest_path, &self.context)
            .map_err(Into::into)
            .map_err(WorkspaceBuildError::CargoWorkspace)
    }
}
