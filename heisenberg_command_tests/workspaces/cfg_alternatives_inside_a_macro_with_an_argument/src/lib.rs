macro_rules! platform_item {
    ($name:ident) => {
        #[cfg(unix)]
        pub struct $name;
        #[cfg(windows)]
        pub struct $name;
    };
}

platform_item!(Platform);
