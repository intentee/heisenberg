use clap::Subcommand;

use crate::check_command::CheckCommand;

#[derive(Debug, Subcommand)]
pub enum HeisenbergSubcommand {
    #[command(
        about = "Report module dependency cycles and structural violations in every crate of a Cargo workspace, for every supported target",
        long_about = "Report module dependency cycles and structural violations in every crate of a Cargo workspace, for x86_64-pc-windows-msvc and x86_64-unknown-linux-gnu.\n\nPrecondition: rustc 1.99.1 compiles the workspace for every supported target, every workspace member lies inside the workspace root, and the cargo home holds every package of Cargo.lock (`cargo fetch`).\n\nThe analysis enables all features, test and debug assertions. Violations: paths, macro calls, modules and extern crates that do not resolve; invalid macro definitions; macro expansions that fail or cannot be parsed; built-in macros that are not implemented; configuration predicates that negate a feature, test or debug_assertions; code active on no supported target; glob imports; re-exports; a second public item in a module; an impl block outside the file of the struct, enum or union it implements, or for a type of another crate; an integration test target holding anything but tests and imports; a module file shared by several modules; packages that depend on each other through dev-dependencies; and a tests-only package whose tests need no test library.\n\nShared module files are analysed in every crate that includes them. Procedural macros and build scripts never run: items under procedural macro attributes are analysed as written, text written outside a crate's module files, such as in other crates or in included files, is trusted, and results reflect the files on disk, including generated ones."
    )]
    Check(CheckCommand),
}
