use crate::analysis_thread::AnalysisThread;

#[must_use]
pub fn joined<TResult, TError>(
    running: Vec<Result<AnalysisThread<'_, Result<TResult, TError>>, TError>>,
) -> Vec<Result<TResult, TError>> {
    running
        .into_iter()
        .map(|thread| thread.and_then(AnalysisThread::join))
        .collect()
}
