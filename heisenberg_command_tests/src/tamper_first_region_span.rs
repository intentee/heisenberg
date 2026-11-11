use serde_json::Value;

use heisenberg_coverage_export::region_position::RegionPosition;
use heisenberg_coverage_export::region_span::RegionSpan;

use crate::constants::FIRST_REGION_COLUMN_POINTER;
use crate::constants::FIRST_REGION_END_COLUMN_POINTER;
use crate::constants::FIRST_REGION_END_LINE_POINTER;
use crate::constants::FIRST_REGION_LINE_POINTER;
use crate::heisenberg_check_error::HeisenbergCheckError;
use crate::tamper_export_value::tamper_export_value;

/// # Errors
///
/// Returns [`HeisenbergCheckError::CoverageExportValueMissing`] when the export has no first
/// region to tamper with.
pub fn tamper_first_region_span(
    export: &mut Value,
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
    tamper_export_value(export, FIRST_REGION_LINE_POINTER, Value::from(start_line))
        .and_then(|()| {
            tamper_export_value(
                export,
                FIRST_REGION_COLUMN_POINTER,
                Value::from(start_column),
            )
        })
        .and_then(|()| {
            tamper_export_value(export, FIRST_REGION_END_LINE_POINTER, Value::from(end_line))
        })
        .and_then(|()| {
            tamper_export_value(
                export,
                FIRST_REGION_END_COLUMN_POINTER,
                Value::from(end_column),
            )
        })
}
