use heisenberg_project_model::nextest_review::NextestReview;
use heisenberg_project_model::package_violation::PackageViolation;

pub struct ToolReview {
    pub clippy_violations: Vec<PackageViolation>,
    pub nextest: NextestReview,
}
