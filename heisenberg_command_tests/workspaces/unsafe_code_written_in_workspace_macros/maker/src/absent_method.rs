#[macro_export]
macro_rules! absent_method {
    ($receiver:expr) => {
        $receiver.absent()
    };
}
