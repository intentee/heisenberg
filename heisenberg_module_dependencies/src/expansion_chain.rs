use std::iter;

use ra_ap_hir::FileRangeWrapper;
use ra_ap_hir::HirFileId;
use ra_ap_hir::InFile;
use ra_ap_hir::MacroCallId;
use ra_ap_hir::Semantics;
use ra_ap_hir_expand::FileRange;
use ra_ap_hir_expand::Lookup;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::ast::HasName;

use heisenberg_module_graph::folded_finding::FoldedFinding;
use heisenberg_module_graph::folded_finding_kind::FoldedFindingKind;
use heisenberg_module_graph::source_location::SourceLocation;

use crate::macro_expansion::MacroExpansion;
use crate::source_locator::SourceLocator;

pub struct ExpansionChain<'chain, 'database> {
    pub call: MacroCallId,
    pub locator: &'chain SourceLocator<'chain>,
    pub semantics: &'chain Semantics<'database, RootDatabase>,
}

impl ExpansionChain<'_, '_> {
    pub fn findings(&self) -> Vec<FoldedFinding> {
        let database = self.semantics.db;
        let calls: Vec<MacroCallId> = iter::successors(Some(self.call), |call| {
            call.lookup(database).kind.file_id().macro_file()
        })
        .collect();

        calls
            .iter()
            .rev()
            .enumerate()
            .filter_map(|(depth, call)| {
                let location = call.lookup(database);

                location
                    .def
                    .ast_id()
                    .either(
                        |definition| definition.to_node(database).name(),
                        |definition| definition.to_node(database).name(),
                    )
                    .map(|name| FoldedFinding {
                        kind: FoldedFindingKind::MacroExpansion {
                            depth,
                            name: name.text().to_owned(),
                        },
                        location: self.locate(match location.kind.file_id() {
                            HirFileId::FileId(_) => {
                                location.kind.original_call_range(database, location.krate)
                            }
                            HirFileId::MacroFile(_) => {
                                location
                                    .to_node(database)
                                    .map(|node| node.text_range())
                                    .original_node_file_range(database)
                                    .0
                            }
                        }),
                    })
            })
            .chain(
                MacroExpansion {
                    call: self.call,
                    database,
                }
                .syntax_errors()
                .into_iter()
                .map(|error| FoldedFinding {
                    kind: FoldedFindingKind::SyntaxError {
                        message: error.to_string(),
                    },
                    location: self.locate(
                        InFile::new(HirFileId::from(self.call), error.range())
                            .original_node_file_range(database)
                            .0,
                    ),
                }),
            )
            .collect()
    }

    fn locate(&self, FileRange { file_id, range }: FileRange) -> SourceLocation {
        self.locator.locate(FileRangeWrapper {
            file_id: file_id.file_id(self.semantics.db),
            range,
        })
    }
}
