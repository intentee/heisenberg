use clap::Subcommand;

use crate::check_command::CheckCommand;

#[derive(Debug, Subcommand)]
pub enum HeisenbergSubcommand {
    #[command(
        about = "Report module dependency cycles and ambiguous code in every crate of a Cargo workspace, for every supported target",
        long_about = "Report module dependency cycles and ambiguous code in every crate of a Cargo workspace, for x86_64-pc-windows-msvc and x86_64-unknown-linux-gnu.\n\nPrecondition: rustc 1.99.1 compiles the workspace for every supported target, every workspace member lies inside the workspace root, and the cargo home holds every package of Cargo.lock (`cargo fetch`).\n\nThe analysis enables all features, test and debug assertions; configuration predicates that negate a feature, test or debug_assertions are violations, and so are glob imports and module files shared by several modules. Procedural macros and build scripts never run: items under procedural macro attributes are analysed as written, text written outside a crate's module files, such as in other crates or in included files, is trusted, and results reflect the files on disk, including generated ones."
    )]
    Check(CheckCommand),
}
