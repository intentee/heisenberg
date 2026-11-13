use std::error::Error;
use std::thread;
use std::thread::Builder;

use heisenberg_analysis_threads::analysis_threads::AnalysisThreads;
use heisenberg_analysis_threads::constants::UNRESERVABLE_STACK_SIZE;

#[test]
fn thread_without_a_reservable_stack_fails_to_start() {
    thread::scope(|scope| {
        let refused = Builder::new()
            .stack_size(UNRESERVABLE_STACK_SIZE)
            .spawn_scoped(scope, || ())
            .err()
            .map(|error| error.to_string())
            .expect("the operating system must refuse a stack it cannot reserve");
        let failure = AnalysisThreads {
            scope,
            stack_size: UNRESERVABLE_STACK_SIZE,
        }
        .spawn(|| ())
        .err()
        .map(|error| {
            [
                error.to_string(),
                error.source().map_or_else(String::new, ToString::to_string),
            ]
        });

        assert_eq!(
            failure,
            Some([
                "the operating system cannot start a thread".to_owned(),
                refused
            ])
        );
    });
}
