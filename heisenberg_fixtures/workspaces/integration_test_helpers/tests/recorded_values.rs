#[derive(Default)]
struct RecordedValues {
    values: Vec<u8>,
}

impl RecordedValues {
    fn record(&mut self, value: u8) {
        self.values.push(value);
    }
}

#[test]
fn recorded_values_keep_their_order() {
    let mut recorded = RecordedValues::default();

    recorded.record(1);
    recorded.record(2);

    assert_eq!(recorded.values, vec![1, 2]);
}
