use std::path::Path;

use serde_json::Value;

use heisenberg_coverage_export::region_position::RegionPosition;
use heisenberg_coverage_export::region_span::RegionSpan;

use crate::constants::EXPORTED_DATA_POINTER;
use crate::constants::EXPORTED_FUNCTIONS_KEY;
use crate::constants::FUNCTION_FIRST_FILE_POINTER;
use crate::constants::FUNCTION_FIRST_REGION_POINTER;
use crate::heisenberg_check_error::HeisenbergCheckError;

/// # Errors
///
/// Returns [`HeisenbergCheckError::CoverageExportFunctionMissing`] when the export lists no
/// function of `file` with a region to tamper with.
pub fn tamper_first_region_span_of_file(
    export: &mut Value,
    file: &Path,
    RegionSpan {
        start:
            RegionPosition {
                line: start_line,
                column: start_column,
            },
        end: RegionPosition {
            line: end_line,
            column: end_column,
        },
    }: RegionSpan,
) -> Result<(), HeisenbergCheckError> {
    export
        .pointer_mut(EXPORTED_DATA_POINTER)
        .and_then(|data| data.get_mut(EXPORTED_FUNCTIONS_KEY))
        .and_then(Value::as_array_mut)
        .and_then(|functions| {
            functions.iter_mut().find(|function| {
                function
                    .pointer(FUNCTION_FIRST_FILE_POINTER)
                    .and_then(Value::as_str)
                    .is_some_and(|listed| Path::new(listed) == file)
            })
        })
        .and_then(|function| function.pointer_mut(FUNCTION_FIRST_REGION_POINTER))
        .and_then(Value::as_array_mut)
        .map(|region| {
            for (position, value) in
                region
                    .iter_mut()
                    .zip([start_line, start_column, end_line, end_column])
            {
                *position = Value::from(value);
            }
        })
        .ok_or_else(|| HeisenbergCheckError::CoverageExportFunctionMissing {
            file: file.to_path_buf(),
        })
}
