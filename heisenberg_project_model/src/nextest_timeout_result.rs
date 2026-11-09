use serde::Deserialize;

#[derive(Debug, Default, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NextestTimeoutResult {
    #[default]
    Fail,
    Pass,
}
