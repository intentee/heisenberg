use std::env;
use std::ffi::OsString;
use std::fs;
#[cfg(unix)]
use std::os::unix::ffi::OsStringExt;
#[cfg(windows)]
use std::os::windows::ffi::OsStringExt;
use std::path::Path;

#[cfg(unix)]
fn non_utf8_name() -> OsString {
    OsString::from_vec(vec![0xff])
}

#[cfg(windows)]
fn non_utf8_name() -> OsString {
    OsString::from_wide(&[0xd800])
}

fn main() {
    let out_directory = env::var("OUT_DIR").expect("cargo sets OUT_DIR for build scripts");

    fs::write(
        Path::new(&out_directory).join("included.rs"),
        "pub struct Included;\n",
    )
    .expect("the build script writes into OUT_DIR");
    fs::write(
        Path::new(&out_directory).join(non_utf8_name()),
        "pub struct Unnamed;\n",
    )
    .expect("the build script writes a file whose name is not UTF-8");
}
