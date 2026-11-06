#[test]
fn contained_operation_reports_its_error() {
    let error = OperationError::Panicked("operation".to_owned());

    assert_eq!(format!("{error:?}"), "Panicked(\"operation\")");
}

#[derive(Debug)]
enum OperationError {
    Panicked(String),
}
