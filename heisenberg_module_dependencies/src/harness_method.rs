use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use heisenberg_project_model::harness_methods::HarnessMethods;

pub struct HarnessMethod {
    pub name: String,
    pub type_name: &'static str,
}

impl HarnessMethod {
    pub fn is_listed_in(&self, listed: &[HarnessMethods]) -> bool {
        listed.iter().any(|HarnessMethods { methods, type_name }| {
            *type_name == self.type_name && methods.contains(&self.name.as_str())
        })
    }
}

impl Display for HarnessMethod {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let Self { name, type_name } = self;

        write!(formatter, "{type_name}::{name}")
    }
}
