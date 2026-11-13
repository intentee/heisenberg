use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;

pub struct ResolutionCache<TKey, TResolution> {
    resolved: RefCell<HashMap<TKey, TResolution>>,
}

impl<TKey: Clone + Eq + Hash, TResolution: Clone> ResolutionCache<TKey, TResolution> {
    pub fn resolve(&self, key: &TKey, resolve: impl FnOnce() -> TResolution) -> TResolution {
        let known = self.resolved.borrow().get(key).cloned();

        known.unwrap_or_else(|| {
            let resolution = resolve();

            self.resolved
                .borrow_mut()
                .insert(key.clone(), resolution.clone());

            resolution
        })
    }
}

impl<TKey, TResolution> Default for ResolutionCache<TKey, TResolution> {
    fn default() -> Self {
        Self {
            resolved: RefCell::default(),
        }
    }
}
