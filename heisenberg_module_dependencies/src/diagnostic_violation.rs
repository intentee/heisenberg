use heisenberg_module_graph::violation::Violation;
use heisenberg_module_graph::violation_kind::ViolationKind;
use ra_ap_hir::AnyDiagnostic;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstPtr;

use crate::diagnostic_site::DiagnosticSite;
use crate::source_locator::SourceLocator;
use crate::token_origin::TokenOrigin;

pub struct DiagnosticViolation<'violation, 'database> {
    pub diagnostic: &'violation AnyDiagnostic<'database>,
    pub locator: &'violation SourceLocator<'violation>,
    pub semantics: &'violation Semantics<'database, RootDatabase>,
    pub token_origin: &'violation TokenOrigin<'violation, 'database>,
}

impl DiagnosticViolation<'_, '_> {
    pub fn violation(&self) -> Option<Violation> {
        self.site()
            .filter(|site| {
                self.token_origin
                    .is_own_text(site.origin.file_id, site.origin.value)
            })
            .map(|site| Violation {
                kind: site.kind,
                location: self.locator.locate(site.display),
            })
    }

    fn site(&self) -> Option<DiagnosticSite> {
        let semantics = self.semantics;

        match self.diagnostic {
            AnyDiagnostic::MacroDefError(diagnostic) => Some(DiagnosticSite {
                display: semantics.diagnostics_display_range(diagnostic.node.map(Into::into)),
                kind: ViolationKind::InvalidMacroDefinition,
                origin: diagnostic.node.map(AstPtr::text_range),
            }),
            AnyDiagnostic::MacroError(diagnostic) if diagnostic.error => Some(DiagnosticSite {
                display: semantics.diagnostics_display_range_for_range(diagnostic.range),
                kind: ViolationKind::FailedMacroExpansion,
                origin: diagnostic.range,
            }),
            AnyDiagnostic::UnimplementedBuiltinMacro(diagnostic) => Some(DiagnosticSite {
                display: semantics.diagnostics_display_range(diagnostic.node),
                kind: ViolationKind::UnimplementedBuiltinMacro,
                origin: diagnostic.node.map(|node| node.text_range()),
            }),
            AnyDiagnostic::UnresolvedExternCrate(diagnostic) => Some(DiagnosticSite {
                display: semantics.diagnostics_display_range(diagnostic.decl.map(Into::into)),
                kind: ViolationKind::UnresolvedExternCrate,
                origin: diagnostic.decl.map(AstPtr::text_range),
            }),
            AnyDiagnostic::UnresolvedMacroCall(diagnostic) if diagnostic.is_bang => {
                Some(DiagnosticSite {
                    display: semantics.diagnostics_display_range_for_range(diagnostic.range),
                    kind: ViolationKind::UnresolvedMacroCall,
                    origin: diagnostic.range,
                })
            }
            AnyDiagnostic::UnresolvedModule(diagnostic) => Some(DiagnosticSite {
                display: semantics.diagnostics_display_range(diagnostic.decl.map(Into::into)),
                kind: ViolationKind::UnresolvedModule,
                origin: diagnostic.decl.map(AstPtr::text_range),
            }),
            _ => None,
        }
    }
}
