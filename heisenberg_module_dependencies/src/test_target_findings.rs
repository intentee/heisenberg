use ra_ap_hir::Adt;

use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_module_graph::folded_finding::FoldedFinding;
use heisenberg_module_graph::folded_finding_kind::FoldedFindingKind;

use crate::adt_violation::AdtViolation;

#[derive(Default)]
pub struct TestTargetFindings {
    pub accepted: Vec<Adt>,
    pub impl_blocks: Vec<AdtViolation>,
    pub types: Vec<AdtViolation>,
}

impl TestTargetFindings {
    pub fn fold(self, CrateModuleGraph { violations, .. }: &mut CrateModuleGraph) {
        let Self {
            accepted,
            impl_blocks,
            types,
        } = self;

        for AdtViolation { adt, violation } in impl_blocks {
            match types.iter().find(
                |AdtViolation {
                     adt: flagged_adt, ..
                 }| *flagged_adt == adt,
            ) {
                Some(flagged) => {
                    violations.fold(
                        flagged.violation.clone(),
                        FoldedFinding {
                            kind: FoldedFindingKind::ImplBlock,
                            location: violation.location,
                        },
                    );
                }
                None if accepted.contains(&adt) => {}
                None => {
                    violations.insert(violation);
                }
            }
        }
    }
}
