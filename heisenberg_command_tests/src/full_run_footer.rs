use crate::constants::HOST_TRIPLE;

#[must_use]
pub fn full_run_footer(crate_targets: usize, modules: usize) -> String {
    format!(
        "Analysed on {HOST_TRIPLE}: {crate_targets} crate targets, {modules} modules\n\
         Precondition: rustc 1.99.1 compiles the workspace for every declared target\n"
    )
}
