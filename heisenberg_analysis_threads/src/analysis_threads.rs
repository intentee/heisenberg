use std::thread::Builder;
use std::thread::Scope;
use std::thread::ScopedJoinHandle;

use ra_ap_stdx::thread::DEFAULT_STACK_SIZE;

pub struct AnalysisThreads<'scope, 'environment> {
    pub scope: &'scope Scope<'scope, 'environment>,
}

impl<'scope> AnalysisThreads<'scope, '_> {
    #[expect(
        clippy::expect_used,
        reason = "spawning fails only when the operating system is out of threads"
    )]
    pub fn spawn<TResult: Send + 'scope>(
        &self,
        work: impl FnOnce() -> TResult + Send + 'scope,
    ) -> ScopedJoinHandle<'scope, TResult> {
        Builder::new()
            .stack_size(DEFAULT_STACK_SIZE)
            .spawn_scoped(self.scope, work)
            .expect("the operating system starts every analysis thread")
    }
}
