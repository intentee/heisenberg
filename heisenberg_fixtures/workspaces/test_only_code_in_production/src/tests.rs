#[test]
fn file_module_runs() {
    assert_eq!(crate::run::run(), 2);
}
