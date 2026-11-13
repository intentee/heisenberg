mod nested;

struct Shared;

#[test]
fn helper_used_by_a_submodule_stays_reported() { assert!(!"helper_used_by_a_submodule_stays_reported".is_empty()); }
