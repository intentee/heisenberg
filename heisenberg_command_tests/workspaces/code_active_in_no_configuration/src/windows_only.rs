pub struct WindowsOnly;

impl WindowsOnly {
    #[cfg(unix)]
    pub fn unix_inside_windows() {}
}
