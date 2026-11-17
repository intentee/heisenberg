use std::collections::HashSet;

use ra_ap_hir::Crate;
use ra_ap_hir::Module;
use ra_ap_syntax::TextRange;
use ra_ap_vfs::FileId;

pub enum ReferenceScope {
    Crates(HashSet<Crate>),
    ModuleTree(Module),
    Range { file: FileId, range: TextRange },
}
