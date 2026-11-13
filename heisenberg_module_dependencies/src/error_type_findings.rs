use std::collections::HashMap;

use ra_ap_hir::Impl;

use heisenberg_module_graph::error_violation::ErrorViolation;
use heisenberg_module_graph::module_findings::ModuleFindings;
use heisenberg_module_graph::source_location::SourceLocation;
use heisenberg_module_graph::violation::Violation;
use heisenberg_module_graph::violation_kind::ViolationKind;

#[derive(Default)]
pub struct ErrorTypeFindings {
    pub implementations: HashMap<Impl, SourceLocation>,
}

impl ErrorTypeFindings {
    pub fn fold(self, ModuleFindings { violations, .. }: &mut ModuleFindings) {
        let Self { implementations } = self;
        let count = implementations.len();

        if count > 1 {
            violations.extend(implementations.into_values().map(|location| Violation {
                kind: ViolationKind::Error(ErrorViolation::MultipleTypes { count }),
                location,
            }));
        }
    }

    pub fn merge(&mut self, Self { implementations }: Self) {
        self.implementations.extend(implementations);
    }
}
