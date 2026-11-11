use ra_ap_paths::AbsPath;
use serde::Deserialize;

use crate::constants::EXPORT_FORMAT_VERSION;
use crate::constants::EXPORT_TYPE;
use crate::coverage_export_error::CoverageExportError;

#[derive(Deserialize)]
pub struct ExportHeader {
    #[serde(rename = "type")]
    pub export_type: String,
    pub version: String,
}

impl ExportHeader {
    pub fn supported(self, export: &AbsPath) -> Result<(), CoverageExportError> {
        let Self {
            export_type,
            version,
        } = self;

        if export_type == EXPORT_TYPE && version == EXPORT_FORMAT_VERSION {
            Ok(())
        } else {
            Err(CoverageExportError::UnsupportedFormat {
                export: export.to_path_buf(),
                export_type,
                version,
            })
        }
    }
}
