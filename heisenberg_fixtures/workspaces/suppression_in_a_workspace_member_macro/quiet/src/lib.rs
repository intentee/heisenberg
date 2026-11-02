#[macro_export]
macro_rules! quiet {
    ($($body:tt)*) => {
        #[allow(clippy::unwrap_used)]
        pub fn quieted() {
            $($body)*
        }
    };
}
