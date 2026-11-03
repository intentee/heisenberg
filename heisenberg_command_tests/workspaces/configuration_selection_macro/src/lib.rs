cfg_select! {
    unix => {
        pub fn platform() -> u8 {
            1
        }
    }
    _ => {
        pub fn platform() -> u8 {
            2
        }
    }
}
