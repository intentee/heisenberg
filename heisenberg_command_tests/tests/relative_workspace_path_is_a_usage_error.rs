use std::path::Path;

use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;
use heisenberg_test_harness::test_harness_error::TestHarnessError;

#[test]
fn relative_workspace_path_is_a_usage_error() {
    let Err(TestHarnessError::Arguments(error)) = (HeisenbergCheck {
        workspace: Path::new("relative"),
    })
    .run() else {
        panic!("a relative workspace path must be rejected");
    };

    assert_eq!(
        error.to_string(),
        "error: invalid value 'relative' for '--workspace <WORKSPACE>': `relative` is not an absolute path\n\n\
         For more information, try '--help'.\n"
    );
    assert_eq!(error.exit_code(), 2);
}
