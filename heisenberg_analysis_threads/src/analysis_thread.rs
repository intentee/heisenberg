use std::panic;
use std::thread::ScopedJoinHandle;

pub struct AnalysisThread<'scope, TResult> {
    pub handle: ScopedJoinHandle<'scope, TResult>,
}

impl<TResult> AnalysisThread<'_, TResult> {
    #[must_use]
    pub fn join(self) -> TResult {
        self.handle
            .join()
            .unwrap_or_else(|payload| panic::resume_unwind(payload))
    }
}
