use crate::constants::LEFT_OUT_CONFIGURATION;
use crate::constants::PARTIAL_RUN_FOOTER_END;
use crate::host_analysis_line::host_analysis_line;

#[must_use]
pub fn host_footer(crate_targets: usize, modules: usize) -> String {
    format!(
        "{}Not analysed: {LEFT_OUT_CONFIGURATION}\n{PARTIAL_RUN_FOOTER_END}",
        host_analysis_line(crate_targets, modules)
    )
}
