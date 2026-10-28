#[cfg(not(feature = "optional"))]
pub fn without_optional() {}

#[cfg_attr(not(test), must_use)]
pub fn attributed() -> u8 {
    1
}

#[cfg(all(unix, not(debug_assertions)))]
pub fn release_unix() {}

#[cfg_attr(unix, cfg(not(feature = "optional")))]
pub fn unix_without_optional() {}

#[cfg(any(windows, unix))]
pub fn any_platform() {}

#[cfg(not(windows))]
pub fn not_windows() {}
