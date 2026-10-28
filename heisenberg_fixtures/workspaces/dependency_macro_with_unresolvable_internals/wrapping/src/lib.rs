#[macro_export]
macro_rules! wrap {
    ($value:expr) => {{
        mod __wrapping_util {
            $crate::declare!();
        }

        let _marker = __wrapping_util::Marker;

        $value
    }};
}

#[cfg(wrapping_generated)]
#[macro_export]
macro_rules! declare {
    () => {
        pub struct Marker;
    };
}
