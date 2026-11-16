#[cfg(feature = "labelled")]
const LABEL: &str = "labelled";

const TITLE: &str = "report";

use std::fmt::Debug;

pub fn report(value: &impl Debug) -> String {
    let mut text = format!("{TITLE} {value:?}");

    #[cfg(feature = "labelled")]
    text.push_str(LABEL);

    text
}
