use std::thread::Builder;
use std::thread::Scope;
use std::thread::ScopedJoinHandle;

use ra_ap_stdx::thread::DEFAULT_STACK_SIZE;

use crate::analysis_threads_error::AnalysisThreadsError;

pub struct AnalysisThreads<'scope, 'environment> {
    pub scope: &'scope Scope<'scope, 'environment>,
}

impl<'scope> AnalysisThreads<'scope, '_> {
    pub fn spawn<TResult: Send + 'scope>(
        &self,
        work: impl FnOnce() -> TResult + Send + 'scope,
    ) -> Result<ScopedJoinHandle<'scope, TResult>, AnalysisThreadsError> {
        Builder::new()
            .stack_size(DEFAULT_STACK_SIZE)
            .spawn_scoped(self.scope, work)
            .map_err(AnalysisThreadsError::from)
    }
}
