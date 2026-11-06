#[test]
fn records() {}

pub struct Recorder;

#[cfg(unix)]
impl Recorder {
    fn record() {}
}
