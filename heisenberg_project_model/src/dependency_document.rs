use serde_json::Value;

pub struct DependencyDocument {
    pub json: Value,
    pub package_id: String,
}
