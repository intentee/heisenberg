use crate::dependency_consumer::DependencyConsumer;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DependencyUse {
    pub consumer: DependencyConsumer,
    pub extern_crate_name: String,
}
