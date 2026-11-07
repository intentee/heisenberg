#[warn(missing_docs)]
fn missing_docs_raised() {}

#[warn(clippy::pedantic)]
fn pedantic_lowered() {}

#[warn(warnings)]
fn warnings_lowered() {}

#[warn(missing_docs,)]
fn trailing_comma_raised() {}

#[warn(clippy::pedantic::all)]
fn nested_lint_path_raised() {}
