use std::str::FromStr;

pub struct Word {
    pub text: String,
}

impl FromStr for Word {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        if text.is_empty() {
            Err(text.to_owned())
        } else {
            Ok(Self {
                text: text.to_owned(),
            })
        }
    }
}
