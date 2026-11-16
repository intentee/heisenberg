use std::path::Path;

use heisenberg_command_tests::cleared_path::cleared_path;
use heisenberg_command_tests::constants::PACKAGE_DIRECTORY;

#[test]
fn path_below_a_file_cannot_be_cleared() {
    let below_a_file = Path::new(PACKAGE_DIRECTORY)
        .join("Cargo.toml")
        .join("below");

    assert_eq!(
        cleared_path(&below_a_file)
            .err()
            .map(|error| error.to_string()),
        Some(format!(
            "`{}` cannot be created to be cleared",
            below_a_file.display()
        ))
    );
}
