#[macro_export]
macro_rules! absent_value {
    () => {
        $crate::absent::value()
    };
}

#[macro_export]
macro_rules! absent_method {
    () => {
        0_u8.absent()
    };
}

#[macro_export]
macro_rules! absent_macro {
    () => {
        $crate::absent!()
    };
}
