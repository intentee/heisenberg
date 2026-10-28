use serde_json::Value;

pub struct PackageDocument {
    pub id: String,
    pub json: Value,
}
