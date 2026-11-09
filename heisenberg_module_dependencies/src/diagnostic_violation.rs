use ra_ap_hir::AnyDiagnostic;
use ra_ap_hir::FileRangeWrapper;
use ra_ap_hir::InFile;
use ra_ap_hir::Semantics;
use ra_ap_hir::UnresolvedMacroCall;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::AstPtr;
use ra_ap_vfs::FileId;

use heisenberg_module_graph::violation_kind::ViolationKind;

use crate::covering_ancestors::CoveringAncestors;
use crate::diagnostic_site::DiagnosticSite;
use crate::item_focus::ItemFocus;
use crate::located_violation::LocatedViolation;
use crate::unresolved_import_binding::UnresolvedImportBinding;

pub struct DiagnosticViolation<'violation, 'database> {
    pub diagnostic: &'violation AnyDiagnostic<'database>,
    pub semantics: &'violation Semantics<'database, RootDatabase>,
}

impl DiagnosticViolation<'_, '_> {
    pub fn site(&self) -> Option<DiagnosticSite> {
        let semantics = self.semantics;

        match self.diagnostic {
            AnyDiagnostic::MacroDefError(diagnostic) => Some(DiagnosticSite {
                origin: diagnostic.node.map(AstPtr::text_range),
                violation: LocatedViolation {
                    display: self.focus(diagnostic.node),
                    kind: ViolationKind::InvalidMacroDefinition,
                },
            }),
            AnyDiagnostic::MacroError(diagnostic) if diagnostic.error => Some(DiagnosticSite {
                origin: diagnostic.range,
                violation: LocatedViolation {
                    display: semantics.diagnostics_display_range_for_range(diagnostic.range),
                    kind: ViolationKind::FailedMacroExpansion,
                },
            }),
            AnyDiagnostic::UnimplementedBuiltinMacro(diagnostic) => Some(DiagnosticSite {
                origin: diagnostic.node.map(|node| node.text_range()),
                violation: LocatedViolation {
                    display: semantics.diagnostics_display_range(diagnostic.node),
                    kind: ViolationKind::UnimplementedBuiltinMacro,
                },
            }),
            AnyDiagnostic::UnresolvedExternCrate(diagnostic) => Some(DiagnosticSite {
                origin: diagnostic.decl.map(AstPtr::text_range),
                violation: LocatedViolation {
                    display: self.focus(diagnostic.decl),
                    kind: ViolationKind::UnresolvedExternCrate,
                },
            }),
            AnyDiagnostic::UnresolvedMacroCall(diagnostic)
                if !self.is_bound_by_unresolved_import(diagnostic) =>
            {
                Some(DiagnosticSite {
                    origin: diagnostic.range,
                    violation: LocatedViolation {
                        display: semantics.diagnostics_display_range_for_range(diagnostic.range),
                        kind: ViolationKind::UnresolvedMacroCall,
                    },
                })
            }
            AnyDiagnostic::UnresolvedModule(diagnostic) => Some(DiagnosticSite {
                origin: diagnostic.decl.map(AstPtr::text_range),
                violation: LocatedViolation {
                    display: self.focus(diagnostic.decl),
                    kind: ViolationKind::UnresolvedModule,
                },
            }),
            _ => None,
        }
    }

    fn focus<TNode: AstNode>(&self, pointer: InFile<AstPtr<TNode>>) -> FileRangeWrapper<FileId> {
        ItemFocus {
            node: self.semantics.to_node(pointer).syntax(),
            semantics: self.semantics,
        }
        .original_range()
    }

    fn is_bound_by_unresolved_import(
        &self,
        UnresolvedMacroCall { path, range, .. }: &UnresolvedMacroCall,
    ) -> bool {
        let semantics = self.semantics;

        path.as_ident().is_some_and(|name| {
            CoveringAncestors {
                range: *range,
                semantics,
            }
            .nodes()
            .next()
            .and_then(|node| semantics.scope(&node))
            .is_some_and(|scope| {
                UnresolvedImportBinding {
                    name,
                    scope: &scope,
                }
                .binds()
            })
        })
    }
}
