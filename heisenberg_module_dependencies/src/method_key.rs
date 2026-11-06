use heisenberg_module_graph::method_group::MethodGroup;

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct MethodKey {
    pub group: MethodGroup,
    pub name: String,
}
