#[macro_export]
macro_rules! call_home {
    () => {
        crate::home::run()
    };
}
