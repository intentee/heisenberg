#[derive(Debug)]
enum OperationError {
    Failed(u8),
}

struct Recorded {
    values: Vec<u8>,
}

impl Recorded {
    fn first(&self) -> Option<u8> {
        self.values.first().copied()
    }
}

fn failure() -> OperationError {
    OperationError::Failed(1)
}

#[test]
fn local_helpers_serve_their_own_test() {
    let recorded = Recorded { values: vec![1] };

    assert_eq!(recorded.first(), Some(1));
    assert_eq!(format!("{:?}", failure()), "Failed(1)");
}
