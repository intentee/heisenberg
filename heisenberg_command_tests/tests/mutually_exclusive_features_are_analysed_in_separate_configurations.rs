use std::path::Path;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::FULL_RUN_FOOTER_END;
use heisenberg_command_tests::constants::HOST_CONFIGURATION;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_only_fixture::HostOnlyFixture;

#[test]
fn mutually_exclusive_features_are_analysed_in_separate_configurations() {
    let outcome = HeisenbergCheck {
        workspace: &HostOnlyFixture {
            directory: Path::new(env!("CARGO_TARGET_TMPDIR")),
            name: "mutually_exclusive_features",
        }
        .path()
        .expect("the host-only fixture must be prepared"),
    }
    .run_with_arguments(&[
        "--configuration",
        &format!("{HOST_CONFIGURATION}-shared"),
        "--configuration",
        &format!("{HOST_CONFIGURATION}-static"),
    ])
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(format!("ModuleCycle: 2 violations\n\
            Modules that depend on each other cannot be understood or changed in isolation. Break the cycle so the modules form layers.\n\
            \n\
            - src/linked/archive.rs:4:25: Parts of crate::linked depend on each other\n        \
            pub fn symbols() -> crate::linked::symbols::Symbols {{\n      \
            crate::linked::archive -> crate::linked::symbols:\n        \
            crate::linked::archive uses crate::linked::symbols at src/linked/archive.rs:4:25\n        \
            crate::linked::archive uses crate::linked::symbols at src/linked/archive.rs:5:9\n      \
            crate::linked::symbols -> crate::linked::archive:\n        \
            crate::linked::symbols uses crate::linked::archive at src/linked/symbols.rs:4:25\n        \
            crate::linked::symbols uses crate::linked::archive at src/linked/symbols.rs:5:9\n\
            - src/shared/loader.rs:4:25: Parts of crate::shared depend on each other\n        \
            pub fn session() -> crate::shared::session::Session {{\n      \
            crate::shared::loader -> crate::shared::session:\n        \
            crate::shared::loader uses crate::shared::session at src/shared/loader.rs:4:25\n        \
            crate::shared::loader uses crate::shared::session at src/shared/loader.rs:5:9\n      \
            crate::shared::session -> crate::shared::loader:\n        \
            crate::shared::session uses crate::shared::loader at src/shared/session.rs:4:24\n        \
            crate::shared::session uses crate::shared::loader at src/shared/session.rs:5:9\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n\
            Analysed on {HOST_CONFIGURATION}-shared: 1 crate targets, 4 modules\n\
            Analysed on {HOST_CONFIGURATION}-static: 1 crate targets, 4 modules\n\
            {FULL_RUN_FOOTER_END}").as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
