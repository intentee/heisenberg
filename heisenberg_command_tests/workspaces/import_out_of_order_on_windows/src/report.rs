pub struct Report;

#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;

#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
