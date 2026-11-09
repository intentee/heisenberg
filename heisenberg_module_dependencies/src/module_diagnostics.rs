use ra_ap_hir::AnyDiagnostic;
use ra_ap_hir::Module;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;

use crate::diagnostic_site::DiagnosticSite;
use crate::diagnostic_violation::DiagnosticViolation;
use crate::inactive_code_site::InactiveCodeSite;

pub struct ModuleDiagnostics {
    pub inactive_code: Vec<InactiveCodeSite>,
    pub sites: Vec<DiagnosticSite>,
}

impl ModuleDiagnostics {
    pub fn new(semantics: &Semantics<'_, RootDatabase>, module: Module) -> Self {
        let mut diagnostics = Vec::new();

        module.diagnostics(semantics.db, &mut diagnostics, false);

        Self {
            inactive_code: diagnostics
                .iter()
                .filter_map(|diagnostic| match diagnostic {
                    AnyDiagnostic::InactiveCode(inactive) => Some(InactiveCodeSite {
                        cfg: inactive.cfg.clone(),
                        node: inactive.node,
                    }),
                    _ => None,
                })
                .collect(),
            sites: diagnostics
                .iter()
                .filter_map(|diagnostic| {
                    DiagnosticViolation {
                        diagnostic,
                        semantics,
                    }
                    .site()
                })
                .collect(),
        }
    }
}
