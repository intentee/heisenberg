use crate::constants::FULL_RUN_FOOTER_END;
use crate::host_analysis_line::host_analysis_line;

#[must_use]
pub fn full_run_footer(crate_targets: usize, modules: usize) -> String {
    format!(
        "{}{FULL_RUN_FOOTER_END}",
        host_analysis_line(crate_targets, modules)
    )
}
