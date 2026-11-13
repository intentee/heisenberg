use std::panic;
use std::thread;

use heisenberg_analysis_threads::analysis_threads::AnalysisThreads;

#[test]
fn panicking_analysis_thread_resumes_its_panic_in_the_joining_thread() {
    let payload = panic::catch_unwind(|| {
        thread::scope(|scope| {
            AnalysisThreads::new(scope)
                .spawn(|| panic::resume_unwind(Box::new(7_u8)))
                .expect("the analysis thread must start")
                .join()
        })
    })
    .expect_err("the panic must reach the joining thread");

    assert_eq!(payload.downcast_ref::<u8>(), Some(&7));
}
