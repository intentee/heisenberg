use std::fs;
use std::path::Path;

use serde_json::Value;
use serde_json::from_slice;
use serde_json::to_vec;

use crate::failure_in::failure_in;
use crate::heisenberg_check_error::HeisenbergCheckError;
use crate::tampered_export::TamperedExport;

pub struct TamperedCoverageExport<'export> {
    pub copy: &'export Path,
    pub source: &'export Path,
}

impl TamperedCoverageExport<'_> {
    /// # Errors
    ///
    /// Returns the [`HeisenbergCheckError`] that kept the source export from being read and
    /// parsed, or the tampered copy from being written.
    pub fn written<TObserved>(
        &self,
        tamper: impl FnOnce(&mut Value) -> TObserved,
    ) -> Result<TamperedExport<TObserved>, HeisenbergCheckError> {
        self.source_export().and_then(|mut export| {
            let observed = tamper(&mut export);

            to_vec(&export)
                .map_err(failure_in(
                    self.copy.to_path_buf(),
                    HeisenbergCheckError::CoverageExportSerialization,
                ))
                .and_then(|contents| {
                    fs::write(self.copy, contents).map_err(failure_in(
                        self.copy.to_path_buf(),
                        HeisenbergCheckError::CoverageExportWrite,
                    ))
                })
                .map(|()| TamperedExport {
                    observed,
                    path: self.copy.to_path_buf(),
                })
        })
    }

    fn source_export(&self) -> Result<Value, HeisenbergCheckError> {
        fs::read(self.source)
            .map_err(failure_in(
                self.source.to_path_buf(),
                HeisenbergCheckError::CoverageExportRead,
            ))
            .and_then(|contents| {
                from_slice::<Value>(&contents).map_err(failure_in(
                    self.source.to_path_buf(),
                    HeisenbergCheckError::CoverageExportParse,
                ))
            })
    }
}
