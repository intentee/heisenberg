#[macro_export]
macro_rules! exported {
    () => {
        1
    };
}

pub use exported;
