use std::sync::OnceLock;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use rayon::Scope;

pub struct Task<'tasks, TValue, TOutput> {
    pub dependents: OnceLock<Vec<&'tasks Self>>,
    pub output: &'tasks OnceLock<TOutput>,
    pub pending: AtomicUsize,
    pub value: &'tasks TValue,
}

impl<'tasks, TValue: Sync, TOutput: Send + Sync> Task<'tasks, TValue, TOutput> {
    pub fn spawn<'scope, TState: Clone + Send + 'scope>(
        &'tasks self,
        scope: &Scope<'scope>,
        state: TState,
        work: &'scope (impl Fn(&mut TState, &TValue) -> TOutput + Sync),
    ) where
        'tasks: 'scope,
    {
        scope.spawn(move |scope| {
            self.output
                .get_or_init(|| work(&mut state.clone(), self.value));

            for dependent in self.dependents.get_or_init(Vec::new) {
                if dependent.pending.fetch_sub(1, Ordering::AcqRel) == 1 {
                    dependent.spawn(scope, state.clone(), work);
                }
            }
        });
    }
}
