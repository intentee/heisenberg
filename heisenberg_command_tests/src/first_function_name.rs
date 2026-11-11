use serde_json::Value;

use crate::constants::FIRST_FUNCTION_NAME_POINTER;

#[must_use]
pub fn first_function_name(export: &Value) -> Option<String> {
    export
        .pointer(FIRST_FUNCTION_NAME_POINTER)
        .and_then(Value::as_str)
        .map(str::to_owned)
}
