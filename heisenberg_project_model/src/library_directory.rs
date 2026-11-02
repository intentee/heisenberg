use ra_ap_paths::AbsPath;
use ra_ap_paths::AbsPathBuf;

pub fn library_directory(sysroot: &AbsPath) -> AbsPathBuf {
    sysroot
        .join("lib")
        .join("rustlib")
        .join("src")
        .join("rust")
        .join("library")
}
