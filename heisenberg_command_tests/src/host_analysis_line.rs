use crate::constants::HOST_CONFIGURATION;

#[must_use]
pub fn host_analysis_line(crate_targets: usize, modules: usize) -> String {
    format!("Analysed on {HOST_CONFIGURATION}: {crate_targets} crate targets, {modules} modules\n")
}
