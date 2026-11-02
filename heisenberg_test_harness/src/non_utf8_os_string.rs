use std::ffi::OsString;
#[cfg(unix)]
use std::os::unix::ffi::OsStringExt;
#[cfg(windows)]
use std::os::windows::ffi::OsStringExt;

#[cfg(unix)]
#[must_use]
pub fn non_utf8_os_string() -> OsString {
    OsString::from_vec(vec![0xff])
}

#[cfg(windows)]
#[must_use]
pub fn non_utf8_os_string() -> OsString {
    OsString::from_wide(&[0xd800])
}
