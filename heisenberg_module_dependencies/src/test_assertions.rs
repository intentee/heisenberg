use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;

use ra_ap_hir::Function;

use crate::test_site::TestSite;

#[derive(Default)]
pub struct TestAssertions {
    asserting: RefCell<HashSet<Function>>,
    comparisons: RefCell<HashMap<Function, Vec<bool>>>,
    runs_tests: Cell<bool>,
    tests: RefCell<Vec<TestSite>>,
}

impl TestAssertions {
    pub fn assert_in(&self, function: Function) {
        self.asserting.borrow_mut().insert(function);
    }

    pub fn compare_in(&self, function: Function, round_trip: bool) {
        self.comparisons
            .borrow_mut()
            .entry(function)
            .or_default()
            .push(round_trip);
    }

    pub fn record(&self, test: TestSite) {
        self.tests.borrow_mut().push(test);
    }

    pub fn round_trip_tests(&self) -> Vec<TestSite> {
        let comparisons = self.comparisons.borrow();

        self.tests
            .borrow()
            .iter()
            .filter(|TestSite { function, .. }| {
                comparisons.get(function).is_some_and(|round_trips| {
                    !round_trips.is_empty() && round_trips.iter().all(|round_trip| *round_trip)
                })
            })
            .cloned()
            .collect()
    }

    pub fn run_test(&self) {
        self.runs_tests.set(true);
    }

    pub const fn runs_tests(&self) -> bool {
        self.runs_tests.get()
    }

    pub fn without_assertions(&self) -> Vec<TestSite> {
        let asserting = self.asserting.borrow();

        self.tests
            .borrow()
            .iter()
            .filter(
                |TestSite {
                     expects_a_panic,
                     function,
                     ..
                 }| !expects_a_panic && !asserting.contains(function),
            )
            .cloned()
            .collect()
    }
}
