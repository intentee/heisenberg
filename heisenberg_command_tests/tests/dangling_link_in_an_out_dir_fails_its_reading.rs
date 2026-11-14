use std::fs;
#[cfg(unix)]
use std::os::unix::fs::symlink as symlink_file;
#[cfg(windows)]
use std::os::windows::fs::symlink_file;
use std::path::Path;

use ra_ap_paths::AbsPathBuf;

use heisenberg_command_tests::cleared_path::cleared_path;
use heisenberg_command_tests::error_chain::error_chain;
use heisenberg_project_model::out_dir_listing::OutDirListing;

#[test]
fn dangling_link_in_an_out_dir_fails_its_reading() {
    let out_dir = AbsPathBuf::assert_utf8(
        Path::new(env!("CARGO_TARGET_TMPDIR"))
            .join("dangling_link_in_an_out_dir_fails_its_reading"),
    );
    let link = out_dir.join("dangling.rs");

    cleared_path(out_dir.as_ref()).expect("the out-dir must be cleared");
    fs::create_dir(&out_dir)
        .and_then(|()| symlink_file(out_dir.join("absent.rs"), &link))
        .expect("the out-dir must hold a dangling link");

    let reason = fs::read(&link)
        .err()
        .map(|error| error.to_string())
        .expect("a dangling link cannot be read");

    assert_eq!(
        OutDirListing {
            out_dir: &out_dir,
            package: "generator",
        }
        .contents()
        .err()
        .map(|error| error_chain(&error)),
        Some(vec![
            format!("the build script output `{link}` cannot be read"),
            reason
        ])
    );
}
