use crate::constants::COVERAGE_FOOTER_END;
use crate::constants::HOST_CONFIGURATION;
use crate::constants::LEFT_OUT_CONFIGURATION;

#[must_use]
pub fn coverage_footer(crate_targets: usize, functions: usize, regions: usize) -> String {
    format!(
        "Measured on {HOST_CONFIGURATION}: {crate_targets} crate targets, {functions} functions, {regions} regions\n\
         Not measured: {LEFT_OUT_CONFIGURATION}\n\
         {COVERAGE_FOOTER_END}"
    )
}
