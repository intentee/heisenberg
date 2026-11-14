use std::fs;
use std::path::Path;

use ra_ap_paths::AbsPathBuf;

use heisenberg_command_tests::cleared_path::cleared_path;
use heisenberg_command_tests::error_chain::error_chain;
use heisenberg_project_model::out_dir_listing::OutDirListing;

#[test]
fn missing_out_dir_fails_its_listing() {
    let missing = AbsPathBuf::assert_utf8(
        Path::new(env!("CARGO_TARGET_TMPDIR")).join("missing_out_dir_fails_its_listing"),
    );

    cleared_path(missing.as_ref()).expect("the out-dir must be cleared");

    let reason = fs::metadata(&missing)
        .err()
        .map(|error| error.to_string())
        .expect("the out-dir must be missing");

    assert_eq!(
        OutDirListing {
            out_dir: &missing,
            package: "generator",
        }
        .contents()
        .err()
        .map(|error| error_chain(&error)),
        Some(vec![
            format!("the build script output in `{missing}` cannot be listed"),
            reason
        ])
    );
}
