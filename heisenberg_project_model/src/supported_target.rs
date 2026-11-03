#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SupportedTarget {
    AArch64AppleDarwin,
    X86_64PcWindowsMsvc,
    X86_64UnknownLinuxGnu,
}

impl SupportedTarget {
    #[must_use]
    pub const fn crates_cfg(self) -> &'static str {
        match self {
            Self::AArch64AppleDarwin => {
                include_str!("../targets/aarch64-apple-darwin/crates.cfg")
            }
            Self::X86_64PcWindowsMsvc => {
                include_str!("../targets/x86_64-pc-windows-msvc/crates.cfg")
            }
            Self::X86_64UnknownLinuxGnu => {
                include_str!("../targets/x86_64-unknown-linux-gnu/crates.cfg")
            }
        }
    }

    #[must_use]
    pub const fn library_cfg(self) -> &'static str {
        match self {
            Self::AArch64AppleDarwin => {
                include_str!("../targets/aarch64-apple-darwin/library.cfg")
            }
            Self::X86_64PcWindowsMsvc => {
                include_str!("../targets/x86_64-pc-windows-msvc/library.cfg")
            }
            Self::X86_64UnknownLinuxGnu => {
                include_str!("../targets/x86_64-unknown-linux-gnu/library.cfg")
            }
        }
    }

    #[must_use]
    pub const fn target_spec(self) -> &'static str {
        match self {
            Self::AArch64AppleDarwin => {
                include_str!("../targets/aarch64-apple-darwin/target-spec.json")
            }
            Self::X86_64PcWindowsMsvc => {
                include_str!("../targets/x86_64-pc-windows-msvc/target-spec.json")
            }
            Self::X86_64UnknownLinuxGnu => {
                include_str!("../targets/x86_64-unknown-linux-gnu/target-spec.json")
            }
        }
    }

    #[must_use]
    pub const fn triple(self) -> &'static str {
        match self {
            Self::AArch64AppleDarwin => "aarch64-apple-darwin",
            Self::X86_64PcWindowsMsvc => "x86_64-pc-windows-msvc",
            Self::X86_64UnknownLinuxGnu => "x86_64-unknown-linux-gnu",
        }
    }
}
