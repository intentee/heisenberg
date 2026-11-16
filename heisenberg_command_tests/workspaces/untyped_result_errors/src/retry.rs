use crate::outcome::Outcome;
use crate::text_outcome::TextOutcome;

pub fn retry(outcome: Outcome) -> TextOutcome {
    outcome.map_err(|()| String::new())
}
