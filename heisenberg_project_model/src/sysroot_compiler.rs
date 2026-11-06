use std::env::consts::EXE_EXTENSION;

use ra_ap_paths::AbsPath;
use ra_ap_paths::AbsPathBuf;
use ra_ap_paths::Utf8Path;

pub fn sysroot_compiler(sysroot: &AbsPath) -> AbsPathBuf {
    sysroot
        .join("bin")
        .join(Utf8Path::new("rustc").with_extension(EXE_EXTENSION))
}
