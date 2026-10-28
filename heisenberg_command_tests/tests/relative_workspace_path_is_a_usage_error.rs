use std::path::Path;

use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn relative_workspace_path_is_a_usage_error() {
    let error = HeisenbergCheck {
        workspace: Path::new("relative"),
    }
    .run()
    .err()
    .expect("a relative workspace path must be rejected");

    assert_eq!(
        error.to_string(),
        "error: invalid value 'relative' for '--workspace <WORKSPACE>': `relative` is not an absolute path\n\n\
         For more information, try '--help'.\n"
    );
    assert_eq!(error.exit_code(), 2);
}
