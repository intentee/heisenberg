use heisenberg_project_model::configuration::Configuration;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ConfigurationFingerprint<'fingerprint> {
    pub configuration: &'fingerprint Configuration,
    pub fingerprint: &'fingerprint [String],
}
