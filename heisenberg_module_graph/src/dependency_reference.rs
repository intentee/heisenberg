use crate::code_category::CodeCategory;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DependencyReference {
    pub category: CodeCategory,
    pub extern_crate_name: String,
}
