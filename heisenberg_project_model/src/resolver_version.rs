use cargo::resolver::ResolveBehavior;
use guppy::graph::cargo::CargoResolverVersion;
use serde_json::from_value;
use serde_json::json;

pub struct ResolverVersion {
    pub behavior: ResolveBehavior,
}

impl ResolverVersion {
    #[expect(
        clippy::expect_used,
        reason = "guppy deserializes every resolver version that cargo reports"
    )]
    pub fn to_guppy(&self) -> CargoResolverVersion {
        from_value(json!(self.behavior.to_manifest()))
            .expect("guppy knows every resolver version cargo reports")
    }
}
