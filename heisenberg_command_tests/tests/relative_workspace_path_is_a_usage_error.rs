use std::path::Path;

use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::heisenberg_check_error::HeisenbergCheckError;

#[test]
fn relative_workspace_path_is_a_usage_error() {
    let error = HeisenbergCheck {
        workspace: Path::new("relative"),
    }
    .run()
    .err();

    assert!(
        matches!(
            &error,
            Some(HeisenbergCheckError::Arguments(arguments))
                if arguments.exit_code() == 2
                    && arguments.to_string()
                        == "error: invalid value 'relative' for '--workspace <WORKSPACE>': `relative` is not an absolute path\n\n\
                            For more information, try '--help'.\n"
        ),
        "{error:?}"
    );
}
