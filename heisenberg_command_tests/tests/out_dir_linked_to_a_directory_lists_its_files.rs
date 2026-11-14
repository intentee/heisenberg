use std::fs;
#[cfg(unix)]
use std::os::unix::fs::symlink as symlink_directory;
#[cfg(windows)]
use std::os::windows::fs::symlink_dir as symlink_directory;
use std::path::Path;

use ra_ap_paths::AbsPathBuf;

use heisenberg_command_tests::cleared_path::cleared_path;
use heisenberg_project_model::out_dir_file::OutDirFile;
use heisenberg_project_model::out_dir_listing::OutDirListing;

#[test]
fn out_dir_linked_to_a_directory_lists_its_files() {
    let directory = AbsPathBuf::assert_utf8(
        Path::new(env!("CARGO_TARGET_TMPDIR"))
            .join("out_dir_linked_to_a_directory_lists_its_files"),
    );
    let target = directory.join("target");
    let link = directory.join("link");

    cleared_path(directory.as_ref()).expect("the scratch directory must be cleared");
    fs::create_dir_all(&target)
        .and_then(|()| fs::write(target.join("generated.rs"), b""))
        .and_then(|()| symlink_directory(&target, &link))
        .expect("the out-dir must link to a directory with one file");

    assert_eq!(
        OutDirListing {
            out_dir: &link,
            package: "generator",
        }
        .contents()
        .map(|contents| {
            contents
                .into_iter()
                .map(|OutDirFile { path, .. }| path)
                .collect::<Vec<AbsPathBuf>>()
        })
        .map_err(|error| error.to_string()),
        Ok(vec![link.join("generated.rs")])
    );
}
