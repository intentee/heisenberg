use cargo::resolver::ResolveBehavior;
use guppy::graph::cargo::CargoResolverVersion;

pub struct ResolverVersion {
    pub behavior: ResolveBehavior,
}

impl ResolverVersion {
    pub const fn to_guppy(&self) -> CargoResolverVersion {
        match self.behavior {
            ResolveBehavior::V1 => CargoResolverVersion::V1,
            ResolveBehavior::V2 => CargoResolverVersion::V2,
            ResolveBehavior::V3 => CargoResolverVersion::V3,
        }
    }
}
