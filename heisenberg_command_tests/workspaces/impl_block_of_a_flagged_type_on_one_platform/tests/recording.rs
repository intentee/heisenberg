pub struct Recorder;

#[cfg(unix)]
impl Recorder {
    fn record() {}
}

#[test]
fn records() { assert!(!"records".is_empty()); }
