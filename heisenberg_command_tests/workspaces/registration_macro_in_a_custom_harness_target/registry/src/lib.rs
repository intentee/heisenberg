#[macro_export]
macro_rules! submit {
    ($registered:ident) => {
        const _: fn() = $registered;
    };
}
