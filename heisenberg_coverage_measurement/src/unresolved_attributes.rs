use ra_ap_hir::AnyDiagnostic;
use ra_ap_hir::InFile;
use ra_ap_hir::Module;
use ra_ap_hir::UnresolvedMacroCall;

use heisenberg_expanded_code::covering_ancestors::CoveringAncestors;
use heisenberg_source_locator::written_range::written_range;

use crate::analysis_context::AnalysisContext;
use crate::coverage_measurement_error::CoverageMeasurementError;

pub struct UnresolvedAttributes<'attributes, 'database> {
    pub context: &'attributes AnalysisContext<'attributes, 'database>,
}

impl UnresolvedAttributes<'_, '_> {
    pub fn verify(&self, module: Module) -> Result<(), CoverageMeasurementError> {
        let mut diagnostics = Vec::new();

        module.diagnostics(
            self.context.test_modules.semantics.db,
            &mut diagnostics,
            false,
        );

        diagnostics
            .iter()
            .try_for_each(|diagnostic| match diagnostic {
                AnyDiagnostic::UnresolvedMacroCall(unresolved) if !unresolved.is_bang => {
                    self.unresolved(unresolved)
                }
                _ => Ok(()),
            })
    }

    fn unresolved(
        &self,
        UnresolvedMacroCall { range, .. }: &UnresolvedMacroCall,
    ) -> Result<(), CoverageMeasurementError> {
        let semantics = self.context.test_modules.semantics;
        let InFile { file_id, value } = *range;

        let in_test_code = CoveringAncestors {
            range: *range,
            semantics,
        }
        .nodes()
        .next()
        .is_some_and(|node| self.context.test_modules.contains(&node));

        if in_test_code {
            Ok(())
        } else {
            self.context
                .locator
                .locate(written_range(semantics.db, file_id, value))
                .and_then(|attribute| {
                    Err(CoverageMeasurementError::UnresolvedAttribute { attribute })
                })
        }
    }
}
