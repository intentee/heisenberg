use ra_ap_hir::Local;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Trace<'database> {
    Decoded(Local<'database>),
    Encoded(Local<'database>),
    Root(Local<'database>),
}

impl Trace<'_> {
    pub const fn decoded(self) -> Option<Self> {
        match self {
            Self::Encoded(value) => Some(Self::Decoded(value)),
            Self::Decoded(_) | Self::Root(_) => None,
        }
    }

    pub const fn encoded(self) -> Option<Self> {
        match self {
            Self::Root(value) => Some(Self::Encoded(value)),
            Self::Decoded(_) | Self::Encoded(_) => None,
        }
    }
}
