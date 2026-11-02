use heisenberg_project_model::supported_target::SupportedTarget;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TargetFingerprint<'fingerprint> {
    pub fingerprint: &'fingerprint [String],
    pub target: SupportedTarget,
}
