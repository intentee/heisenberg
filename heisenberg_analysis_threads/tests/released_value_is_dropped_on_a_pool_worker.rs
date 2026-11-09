use std::num::NonZeroU16;
use std::sync::Arc;
use std::sync::OnceLock;

use rayon::current_thread_index;

use heisenberg_analysis_threads::worker_pool::WorkerPool;

struct DropWitness {
    dropped_on: Arc<OnceLock<Option<usize>>>,
}

impl Drop for DropWitness {
    fn drop(&mut self) {
        self.dropped_on.get_or_init(current_thread_index);
    }
}

#[test]
fn released_value_is_dropped_on_a_pool_worker() {
    let dropped_on = Arc::new(OnceLock::new());

    WorkerPool::start(NonZeroU16::MIN).expect("the worker pool must start");
    WorkerPool::release(DropWitness {
        dropped_on: Arc::clone(&dropped_on),
    });

    assert_eq!(*dropped_on.wait(), Some(0));
}
