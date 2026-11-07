use crate::heisenberg_exit_status::HeisenbergExitStatus;

pub struct RenderedReport {
    pub status: HeisenbergExitStatus,
    pub text: String,
}
