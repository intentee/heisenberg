use heisenberg_module_graph::violation_kind::ViolationKind;

use crate::constants::TYPE_PARAMETER_PREFIX;

pub struct ParameterName<'name> {
    pub identifier: &'name str,
}

impl ParameterName<'_> {
    pub fn single_letter_violation(&self) -> Option<ViolationKind> {
        (self.identifier.chars().count() == 1).then_some(ViolationKind::SingleLetterName)
    }

    pub fn type_parameter_violation(&self) -> Option<ViolationKind> {
        let mut characters = self.identifier.chars();

        self.single_letter_violation().or_else(|| {
            (characters.next() != Some(TYPE_PARAMETER_PREFIX)
                || !characters.next().is_some_and(char::is_uppercase))
            .then_some(ViolationKind::UnprefixedTypeParameter)
        })
    }
}
