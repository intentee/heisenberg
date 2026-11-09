use std::time::Duration;

use humantime::DurationError;
use humantime::parse_duration;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
pub struct NextestDuration {
    pub duration: Duration,
}

impl TryFrom<String> for NextestDuration {
    type Error = DurationError;

    fn try_from(text: String) -> Result<Self, DurationError> {
        parse_duration(&text).map(|duration| Self { duration })
    }
}
