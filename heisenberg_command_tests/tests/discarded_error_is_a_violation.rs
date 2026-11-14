use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

const DISCARDED_ERROR_REPORT: &str = "DiscardedError: 44 violations\n\
    A dropped error hides the failure that caused it, so the program continues on a state it never checked and nothing explains what went wrong. Handle the error: return it, wrap it as the source of a more specific error, or match its variants.\n\
    \n\
    - src/adapted_copied_value.rs:3:42: Error is dropped without being handled\n        \
    text.parse::<u8>().as_ref().copied().ok()\n\
    - src/adapted_temporary_check.rs:3:33: Error is dropped without being handled\n        \
    text.parse::<u8>().as_ref().is_ok()\n\
    - src/adapted_temporary_value.rs:3:33: Error is dropped without being handled\n        \
    text.parse::<u8>().as_ref().ok().copied()\n\
    - src/attempt.rs:11:27: Error is dropped without being handled\n            \
    let Self { value, .. } = self;\n\
    - src/borrowed_into_iterator.rs:3:27: Error is dropped without being handled\n        \
    (&text.parse::<u8>()).into_iter().count()\n\
    - src/borrowed_temporary_iteration.rs:5:18: Error is dropped without being handled\n        \
    for value in &text.parse::<u8>() {\n\
    - src/borrowed_temporary_value.rs:3:12: Error is dropped without being handled\n        \
    if let Ok(value) = &text.parse::<u8>() {\n\
    - src/borrowed_temporary_wildcard.rs:2:9: Error is dropped without being handled\n        \
    let _ = &text.parse::<u8>();\n\
    - src/constant_check.rs:7:13: Error is dropped without being handled\n        \
    CHECKED.is_ok()\n\
    - src/dropped.rs:2:5: Error is dropped without being handled\n        \
    drop(text.parse::<u8>());\n\
    - src/error_variant_ignored.rs:5:13: Error is dropped without being handled\n            \
    Err(_) => 0,\n\
    - src/field_holder.rs:32:40: Error is dropped without being handled\n            \
    Self::parsed_from(text).parsed.is_ok()\n\
    - src/flat_mapped.rs:3:18: Error is dropped without being handled\n        \
    texts.iter().flat_map(|text| text.parse::<u8>()).count()\n\
    - src/flattened.rs:5:24: Error is dropped without being handled\n        \
    parsed.into_iter().flatten().count()\n\
    - src/function_value.rs:6:21: Error is dropped without being handled\n            \
    .filter_map(Result::ok)\n\
    - src/iterated_result.rs:5:18: Error is dropped without being handled\n        \
    for value in text.parse::<u8>() {\n\
    - src/lent_closure_parameter.rs:3:50: Error is dropped without being handled\n        \
    *text.parse::<u8>().as_ref().unwrap_or_else(|_| &0)\n\
    - src/lent_drop.rs:2:5: Error is dropped without being handled\n        \
    drop(text.parse::<u8>().as_ref());\n\
    - src/lent_error_unwrapped.rs:2:9: Error is dropped without being handled\n        \
    let _ = text.parse::<u8>().as_ref().unwrap_err();\n\
    - src/lent_iteration.rs:5:18: Error is dropped without being handled\n        \
    for value in text.parse::<u8>().as_ref() {\n\
    - src/lent_match_arm.rs:5:13: Error is dropped without being handled\n            \
    Err(_) => 0,\n\
    - src/lent_match_catch_all.rs:5:9: Error is dropped without being handled\n            \
    _ => 0,\n\
    - src/lent_match_rest.rs:5:13: Error is dropped without being handled\n            \
    Err(..) => 0,\n\
    - src/lent_nested_assignee.rs:5:6: Error is dropped without being handled\n        \
    (_, value) = (text.parse::<u8>().as_ref(), 1);\n\
    - src/lent_nested_pattern.rs:3:10: Error is dropped without being handled\n        \
    let (_, value) = (text.parse::<u8>().as_ref(), 1);\n\
    - src/lent_underscore.rs:2:5: Error is dropped without being handled\n        \
    _ = text.parse::<u8>().as_ref();\n\
    - src/lent_unread_binding.rs:2:9: Error is dropped without being handled\n        \
    let _parsed = text.parse::<u8>().as_ref();\n\
    - src/lent_value_only.rs:3:12: Error is dropped without being handled\n        \
    if let Ok(value) = text.parse::<u8>().as_ref() {\n\
    - src/lent_value_only_let_else.rs:3:9: Error is dropped without being handled\n        \
    let Ok(value) = text.parse::<u8>().as_ref().copied() else {\n\
    - src/lent_wildcard.rs:2:9: Error is dropped without being handled\n        \
    let _ = text.parse::<u8>().as_ref();\n\
    - src/mutable_iterator.rs:3:24: Error is dropped without being handled\n        \
    text.parse::<u8>().iter_mut().count()\n\
    - src/omitted_array_elements.rs:6:17: Error is dropped without being handled\n        \
    let [first, ..] = parsed;\n\
    - src/omitted_error.rs:5:13: Error is dropped without being handled\n            \
    Err(..) => 0,\n\
    - src/omitted_field.rs:15:25: Error is dropped without being handled\n            \
    let Self(value, ..) = self;\n\
    - src/omitted_tuple_element.rs:3:17: Error is dropped without being handled\n        \
    let (value, ..) = (1, text.parse::<u8>());\n\
    - src/optional_value.rs:3:24: Error is dropped without being handled\n        \
    text.parse::<u8>().ok()\n\
    - src/parenthesized_lender.rs:3:35: Error is dropped without being handled\n        \
    (text.parse::<u8>().as_ref()).ok().copied()\n\
    - src/temporary_check.rs:3:24: Error is dropped without being handled\n        \
    text.parse::<u8>().is_ok()\n\
    - src/underscore_assignment.rs:2:5: Error is dropped without being handled\n        \
    _ = text.parse::<u8>();\n\
    - src/unread_binding.rs:2:9: Error is dropped without being handled\n        \
    let _parsed = text.parse::<u8>();\n\
    - src/value_iterator.rs:3:24: Error is dropped without being handled\n        \
    text.parse::<u8>().into_iter().count()\n\
    - src/value_only_if_let.rs:3:12: Error is dropped without being handled\n        \
    if let Ok(value) = text.parse::<u8>() {\n\
    - src/value_only_let_else.rs:3:9: Error is dropped without being handled\n        \
    let Ok(value) = text.parse::<u8>() else {\n\
    - src/wildcard_binding.rs:2:9: Error is dropped without being handled\n        \
    let _ = text.parse::<u8>();\n\
    \n\
    LintAllowedInManifest: 2 violations\n\
    Allowing a lint is a suppression, and suppressions are a slippery slope. Remove the allow; if the lint then reports code, fix that code.\n\
    \n\
    - Cargo.toml: Package discarded_errors allows dropping_copy_types in its rust lints\n\
    - Cargo.toml: Package discarded_errors allows for_loops_over_fallibles in its rust lints\n\
    \n\
    ------------------------------\n\
    46 errors found\n\
    ------------------------------\n";

#[test]
fn discarded_error_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "discarded_errors",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok([
            DISCARDED_ERROR_REPORT,
            NextestPresenceEvaluation::NotEvaluated
                .host_footer(1, 54)
                .as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
