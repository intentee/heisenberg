use serde::Deserialize;

#[derive(Deserialize)]
pub struct TargetSpec {
    #[serde(rename = "data-layout")]
    pub data_layout: String,
}
