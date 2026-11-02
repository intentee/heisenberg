#[cfg(target_os = "macos")]
const fn open_descriptors_directory_path() -> &'static str {
    "/dev/fd"
}

#[cfg(target_os = "linux")]
const fn open_descriptors_directory_path() -> &'static str {
    "/proc/self/fd"
}

pub fn open_descriptors_directory() -> &'static str {
    open_descriptors_directory_path()
}
