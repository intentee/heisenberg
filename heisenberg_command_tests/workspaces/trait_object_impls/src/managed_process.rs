pub trait ManagedProcess {
    fn name(&self) -> &str;
}

impl dyn ManagedProcess {
    pub fn is_named(&self, name: &str) -> bool {
        self.name() == name
    }
}
