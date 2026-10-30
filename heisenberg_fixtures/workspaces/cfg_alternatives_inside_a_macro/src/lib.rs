macro_rules! platform_item {
    () => {
        #[cfg(unix)]
        pub struct Platform;
        #[cfg(windows)]
        pub struct Platform;
    };
}

platform_item!();
