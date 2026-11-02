use ra_ap_hir::Name;
use ra_ap_hir::ScopeDef;
use ra_ap_hir::SemanticsScope;

pub struct UnresolvedImportBinding<'binding, 'database> {
    pub name: &'binding Name,
    pub scope: &'binding SemanticsScope<'database>,
}

impl UnresolvedImportBinding<'_, '_> {
    pub fn binds(&self) -> bool {
        let mut bound = false;

        self.scope.process_all_names(&mut |candidate, definition| {
            bound |= candidate == *self.name && definition == ScopeDef::Unknown;
        });

        bound
    }
}
