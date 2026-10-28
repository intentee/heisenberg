#[macro_export]
macro_rules! call {
    ($function:path) => {
        $function()
    };
}
