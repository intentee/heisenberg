#[macro_export]
macro_rules! generated_type {
    ($name:ident) => {
        pub struct $name;
    };
}
