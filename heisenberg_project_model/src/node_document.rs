use crate::dependency_document::DependencyDocument;

pub struct NodeDocument {
    pub dependencies: Vec<DependencyDocument>,
    pub id: String,
}
