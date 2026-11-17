#[macro_export]
macro_rules! absent_value {
    () => {
        $crate::absent::value()
    };
}
