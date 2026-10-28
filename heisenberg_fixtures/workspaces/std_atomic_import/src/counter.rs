use std::sync::atomic::AtomicUsize;

pub static COUNTER: AtomicUsize = AtomicUsize::new(0);
