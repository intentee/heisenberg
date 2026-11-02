use ra_ap_hir::InFile;
use ra_ap_syntax::TextRange;

use crate::located_violation::LocatedViolation;

pub struct DiagnosticSite {
    pub origin: InFile<TextRange>,
    pub violation: LocatedViolation,
}
