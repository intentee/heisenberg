use crate::managed_process::ManagedProcess;

impl dyn ManagedProcess {
    pub fn description(&self) -> String {
        format!("process {}", self.name())
    }
}
