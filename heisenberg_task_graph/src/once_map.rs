use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::sync::PoisonError;

pub struct OnceMap<TKey, TValue> {
    cells: Mutex<HashMap<TKey, Arc<OnceLock<TValue>>>>,
}

impl<TKey: Eq + Hash, TValue: Clone> OnceMap<TKey, TValue> {
    pub fn get_or_init(&self, key: TKey, init: impl FnOnce() -> TValue) -> TValue {
        let cell = Arc::clone(
            self.cells
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .entry(key)
                .or_default(),
        );

        cell.get_or_init(init).clone()
    }
}

impl<TKey, TValue> Default for OnceMap<TKey, TValue> {
    fn default() -> Self {
        Self {
            cells: Mutex::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Barrier;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;

    use rayon::ThreadPoolBuilder;
    use rayon::iter::IntoParallelIterator;
    use rayon::iter::ParallelIterator;

    use super::OnceMap;

    #[test]
    fn concurrent_claimants_of_a_key_share_one_computation() {
        let claimants = 8;
        let map: OnceMap<usize, usize> = OnceMap::default();
        let computations = AtomicUsize::new(0);
        let all_claiming = Barrier::new(claimants);
        let values: Vec<usize> = ThreadPoolBuilder::new()
            .num_threads(claimants)
            .build()
            .expect("the test pool must start")
            .install(|| {
                (0..claimants)
                    .into_par_iter()
                    .map(|_| {
                        all_claiming.wait();

                        map.get_or_init(1, || computations.fetch_add(1, Ordering::SeqCst))
                    })
                    .collect()
            });

        assert_eq!(computations.load(Ordering::SeqCst), 1);
        assert_eq!(values, vec![0; claimants]);
    }

    #[test]
    fn each_key_is_computed_once() {
        let map: OnceMap<usize, usize> = OnceMap::default();
        let computations = AtomicUsize::new(0);
        let values: Vec<usize> = [1, 2, 1, 2]
            .into_iter()
            .map(|key| {
                map.get_or_init(key, || {
                    computations.fetch_add(1, Ordering::SeqCst);

                    key
                })
            })
            .collect();

        assert_eq!(values, [1, 2, 1, 2]);
        assert_eq!(computations.load(Ordering::SeqCst), 2);
    }
}
