use std::time::Duration;

use humantime::DurationError;
use humantime::parse_duration;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
pub enum NextestPeriodSetting {
    Inherited,
    Period { period: Duration },
}

impl TryFrom<String> for NextestPeriodSetting {
    type Error = DurationError;

    fn try_from(text: String) -> Result<Self, DurationError> {
        if text.is_empty() {
            Ok(Self::Inherited)
        } else {
            parse_duration(&text).map(|period| Self::Period { period })
        }
    }
}
