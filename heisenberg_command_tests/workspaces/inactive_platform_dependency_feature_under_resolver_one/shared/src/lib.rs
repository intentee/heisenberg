#[cfg(feature = "cyclic")]
#[macro_export]
macro_rules! link {
    () => {
        pub fn second() -> crate::second::Second {
            crate::second::Second
        }
    };
}

#[cfg(not(feature = "cyclic"))]
#[macro_export]
macro_rules! link {
    () => {};
}
