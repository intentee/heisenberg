#[derive(Debug)]
pub fn derived_helper() {}

#[test]
fn derived_helper_runs() {
    derived_helper();
    assert!(!"derived_helper_runs".is_empty()); }
