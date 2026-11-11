use heisenberg_coverage_measurement::measured_coverage::MeasuredCoverage;
use heisenberg_project_model::configuration::Configuration;

pub struct ConfigurationCoverage {
    pub configuration: Configuration,
    pub coverage: MeasuredCoverage,
}
