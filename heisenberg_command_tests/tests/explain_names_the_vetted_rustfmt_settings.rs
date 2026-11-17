use std::collections::BTreeMap;
use std::str;

use clap::Parser;

use heisenberg_command::heisenberg_command::HeisenbergCommand;
use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::EXPLAIN_SUBCOMMAND;
use heisenberg_command_tests::constants::PROGRAM_NAME;

#[test]
fn explain_names_the_vetted_rustfmt_settings() {
    let mut output = Vec::new();
    let mut errors = Vec::new();
    let status = HeisenbergCommand::try_parse_from([
        PROGRAM_NAME,
        EXPLAIN_SUBCOMMAND,
        "UnvettedRustfmtSetting",
    ])
    .expect("the arguments must parse")
    .run(&BTreeMap::new(), &mut output, &mut errors)
    .expect("the errors of the run must be reported");

    assert_eq!(
        str::from_utf8(&output),
        Ok(
            "Rule 3.14 UnvettedRustfmtSetting: A rustfmt configuration setting that Heisenberg has not vetted\n\
        Section 3, Lints and formatting. Scope: workspace. Evaluated in each analysed configuration.\n\
        \n\
        A setting of a rustfmt.toml or .rustfmt.toml, in the workspace root or a directory between the workspace root and a module file of a workspace member, other than the stable settings that only choose a layout. Settings that exempt code from formatting, such as disable_all_formatting, ignore or skip_children, unstable settings and unknown settings are not vetted.\n\
        \n\
        Vetted settings: array_width, attr_fn_like_width, chain_width, edition, fn_args_layout, fn_call_width, fn_params_layout, force_explicit_abi, hard_tabs, hex_literal_case, match_arm_leading_pipes, match_block_trailing_comma, max_width, merge_derives, newline_style, remove_nested_parens, reorder_imports, reorder_modules, short_array_element_width_threshold, single_line_if_else_max_width, single_line_let_else_max_width, struct_lit_width, struct_variant_width, style_edition, tab_spaces, use_field_init_shorthand, use_small_heuristics, use_try_shorthand\n\
        \n\
        Rationale: A rustfmt setting can exempt code from formatting, and an unstable or unknown one is ignored by the supported toolchain.\n\
        \n\
        Remedy: Remove the setting.\n\
        \n\
        See also: RustfmtConfigurationAmbiguous, RustfmtConfigurationInvalid, RustfmtConfigurationMissing, RustfmtConfigurationUnreadable\n\
        "
        )
    );
    assert_eq!(errors, b"");
    assert_eq!(status, HeisenbergExitStatus::Explained);
}
