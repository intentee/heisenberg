use library::report::Report;
use library_checks::rendered;

#[test]
fn report_is_rendered() {
    assert_eq!(rendered(&Report { lines: 1 }), 2);
}
