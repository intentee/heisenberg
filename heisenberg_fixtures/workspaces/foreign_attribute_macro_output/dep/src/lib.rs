#![feature(macro_attr)]

#[macro_export]
macro_rules! companion {
    attr() ($item:item) => {
        $item

        #[cfg(any())]
        pub fn hidden() {}

        pub const GENERATED: u8 = 1;

        pub fn generated() {}
    };
}
