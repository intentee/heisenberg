#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SupportedTarget {
    X86_64PcWindowsMsvc,
    X86_64UnknownLinuxGnu,
}

impl SupportedTarget {
    pub const ALL: [Self; 2] = [Self::X86_64PcWindowsMsvc, Self::X86_64UnknownLinuxGnu];

    #[must_use]
    pub const fn triple(self) -> &'static str {
        match self {
            Self::X86_64PcWindowsMsvc => "x86_64-pc-windows-msvc",
            Self::X86_64UnknownLinuxGnu => "x86_64-unknown-linux-gnu",
        }
    }

    #[must_use]
    pub const fn crates_cfg(self) -> &'static str {
        match self {
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
            Self::X86_64PcWindowsMsvc => {
                include_str!("../targets/x86_64-pc-windows-msvc/library.cfg")
            }
            Self::X86_64UnknownLinuxGnu => {
                include_str!("../targets/x86_64-unknown-linux-gnu/library.cfg")
            }
        }
    }
}
