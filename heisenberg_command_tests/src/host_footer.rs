use crate::constants::HOST_TRIPLE;
use crate::constants::LEFT_OUT_TRIPLE;

#[must_use]
pub fn host_footer(crate_targets: usize, modules: usize) -> String {
    format!(
        "Analysed on {HOST_TRIPLE}: {crate_targets} crate targets, {modules} modules\n\
         Not analysed: {LEFT_OUT_TRIPLE}\n\
         Features: all\n\
         Not evaluated in a partial run: CodeActiveOnNoTarget, the empty-library check of TestsOnlyPackageWithoutTestLibraries\n\
         Precondition: rustc 1.99.1 compiles the workspace for every analysed target with the selected features\n"
    )
}
