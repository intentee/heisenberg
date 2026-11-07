use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeclaredConfiguration {
    pub default_features: bool,
    pub features: Vec<String>,
    pub name: String,
    pub target: String,
}
