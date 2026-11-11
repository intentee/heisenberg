use serde_json::Value;

use crate::heisenberg_check_error::HeisenbergCheckError;

/// # Errors
///
/// Returns [`HeisenbergCheckError::CoverageExportValueMissing`] when the export holds no value
/// at `pointer`.
pub fn tamper_export_value(
    export: &mut Value,
    pointer: &'static str,
    value: Value,
) -> Result<(), HeisenbergCheckError> {
    export
        .pointer_mut(pointer)
        .map(|tampered| *tampered = value)
        .ok_or(HeisenbergCheckError::CoverageExportValueMissing { pointer })
}
