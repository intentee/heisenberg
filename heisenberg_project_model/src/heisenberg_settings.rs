use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeisenbergSettings {
    pub targets: Vec<String>,
}
