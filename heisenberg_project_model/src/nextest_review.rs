use crate::nextest_presence::NextestPresence;
use crate::package_violation::PackageViolation;

pub struct NextestReview {
    pub presence: NextestPresence,
    pub violations: Vec<PackageViolation>,
}
