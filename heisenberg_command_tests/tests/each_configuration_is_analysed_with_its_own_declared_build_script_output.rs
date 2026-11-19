use std::path::Path;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::CONFIGURATION_ARGUMENT;
use heisenberg_command_tests::constants::FULL_RUN_FOOTER_END;
use heisenberg_command_tests::constants::HOST_CONFIGURATION;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_only_fixture::HostOnlyFixture;

#[test]
fn each_configuration_is_analysed_with_its_own_declared_build_script_output() {
    let outcome = HeisenbergCheck {
        workspace: &HostOnlyFixture {
            directory: Path::new(env!("CARGO_TARGET_TMPDIR")),
            name: "out_dir_per_configuration",
        }
        .path()
        .expect("the host-only fixture must be prepared"),
    }
    .run_with_arguments(&[
        CONFIGURATION_ARGUMENT,
        HOST_CONFIGURATION,
        CONFIGURATION_ARGUMENT,
        &format!("{HOST_CONFIGURATION}-accelerated"),
    ])
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(format!(
            "BuildScriptEnvironmentVariable: 1 violation\n\
             Cargo hands such a variable to the compiler after the variables it sets itself, so it reaches every tool that runs in place of the compiler: clippy reads its lint arguments and the location of its configuration from that environment, and a build script can switch off the lints the workspace denies. Write the value into a file in OUT_DIR and bring it in with include!, or declare a fixed value in the [env] table of the cargo configuration.\n\
             \n\
             - Cargo.toml: Declared output of the build script of bindings_sys sets environment variables for the compiler\n      \
             sets DECLARED_VALUE\n\
             \n\
             ------------------------------\n\
             1 errors found in 1 rule\n\
             ------------------------------\n\
             Analysed on {HOST_CONFIGURATION}: 3 crate targets, 5 modules\n\
             Analysed on {HOST_CONFIGURATION}-accelerated: 3 crate targets, 6 modules\n\
             {FULL_RUN_FOOTER_END}"
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
