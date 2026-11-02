use std::thread::ScopedJoinHandle;

pub struct PrimingThread<'scope> {
    pub crates: Vec<String>,
    pub handle: ScopedJoinHandle<'scope, ()>,
}
