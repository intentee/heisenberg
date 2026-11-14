use std::fs;
use std::path::Path;

use ra_ap_paths::AbsPathBuf;

use heisenberg_command_tests::cleared_path::cleared_path;
use heisenberg_command_tests::error_chain::error_chain;
use heisenberg_project_model::out_dir_listing::OutDirListing;

#[test]
fn out_dir_that_is_a_file_fails_its_listing() {
    let directory = AbsPathBuf::assert_utf8(
        Path::new(env!("CARGO_TARGET_TMPDIR")).join("out_dir_that_is_a_file_fails_its_listing"),
    );
    let file = directory.join("out");

    cleared_path(directory.as_ref()).expect("the scratch directory must be cleared");
    fs::create_dir(&directory)
        .and_then(|()| fs::write(&file, b""))
        .expect("the out-dir must be a file");

    assert_eq!(
        OutDirListing {
            out_dir: &file,
            package: "generator",
        }
        .contents()
        .err()
        .map(|error| error_chain(&error)),
        Some(vec![format!(
            "the build script output `{file}` is not a directory"
        )])
    );
}
