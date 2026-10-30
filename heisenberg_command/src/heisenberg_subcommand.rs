use clap::Subcommand;

use heisenberg_project_model::constants::SUPPORTED_TARGETS;

use crate::check_command::CheckCommand;
use crate::constants::SUPPORTED_RUST_VERSION;

#[derive(Debug, Subcommand)]
pub enum HeisenbergSubcommand {
    #[command(
        about = "Report module dependency cycles and structural violations in every crate of a Cargo workspace, for every supported target",
        long_about = format!("Report module dependency cycles and structural violations in every crate of a Cargo workspace, for {}.\n\nPrecondition: rustc {SUPPORTED_RUST_VERSION} compiles the workspace for every supported target, every workspace member lies inside the workspace root, and the cargo home holds every package of Cargo.lock (`cargo fetch`).\n\nThe analysis enables all features, test and debug assertions. Violations: paths, macro calls, modules and extern crates that do not resolve; invalid macro definitions; macro expansions that fail or cannot be parsed; built-in macros that are not implemented; configuration predicates that negate a feature, test or debug_assertions; code active on no supported target; glob imports; re-exports; a second public item in a module, constants aside; a public constant outside the crate's constants module, crate::constants; a public item other than a constant inside it; an impl block outside the file of the struct, enum or union it implements, or for a type of another crate; an integration test target holding anything but tests, constants, imports and modules; a module file shared by several modules, or by two modules of one crate, which leaves the second unanalysed; packages that depend on each other through dev-dependencies; and a tests-only package whose tests need no test library.\n\nEach issue is reported once, as `ERROR: <IssueType> - <file>[:<line>:<column>] - <message>` followed by its source line when it has one; when it holds only in some of the crate targets or platforms that analyse its file, the message names them. Shared module files are analysed in every crate that includes them. The public item, constant and integration test target rules apply to items written in module files; items that macros generate are trusted. Procedural macros and build scripts never run: items under procedural macro attributes are analysed as written, text written outside a crate's module files, such as in other crates or in included files, is trusted, and results reflect the files on disk, including generated ones.", SUPPORTED_TARGETS.map(|target| target.triple()).join(" and "))
    )]
    Check(CheckCommand),
}
