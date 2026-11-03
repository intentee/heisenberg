pub fn outer() {
    #[path = "hidden.rs"]
    mod hidden;

    hidden::run();
}
