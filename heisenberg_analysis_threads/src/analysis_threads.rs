use std::thread::Builder;
use std::thread::Scope;

use ra_ap_stdx::thread::DEFAULT_STACK_SIZE;

use crate::analysis_thread::AnalysisThread;
use crate::analysis_threads_error::AnalysisThreadsError;

pub struct AnalysisThreads<'scope, 'environment> {
    pub scope: &'scope Scope<'scope, 'environment>,
    pub stack_size: usize,
}

impl<'scope, 'environment> AnalysisThreads<'scope, 'environment> {
    #[must_use]
    pub const fn new(scope: &'scope Scope<'scope, 'environment>) -> Self {
        Self {
            scope,
            stack_size: DEFAULT_STACK_SIZE,
        }
    }

    /// # Errors
    ///
    /// Returns [`AnalysisThreadsError::Spawn`] when the operating system cannot start the thread.
    pub fn spawn<TResult: Send + 'scope>(
        &self,
        work: impl FnOnce() -> TResult + Send + 'scope,
    ) -> Result<AnalysisThread<'scope, TResult>, AnalysisThreadsError> {
        Builder::new()
            .stack_size(self.stack_size)
            .spawn_scoped(self.scope, work)
            .map(|handle| AnalysisThread { handle })
            .map_err(AnalysisThreadsError::from)
    }
}
