use serde::Deserialize;
use serde::de::IgnoredAny;

use crate::export_function::ExportFunction;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExportData {
    pub files: IgnoredAny,
    pub functions: Vec<ExportFunction>,
    pub totals: IgnoredAny,
}
