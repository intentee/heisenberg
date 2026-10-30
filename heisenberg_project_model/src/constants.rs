use crate::supported_target::SupportedTarget;

pub const SUPPORTED_TARGETS: [SupportedTarget; 2] = [
    SupportedTarget::X86_64PcWindowsMsvc,
    SupportedTarget::X86_64UnknownLinuxGnu,
];
