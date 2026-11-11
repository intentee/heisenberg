use std::iter;

use ra_ap_hir::FileRangeWrapper;
use ra_ap_hir::HirFileId;
use ra_ap_hir::InFile;
use ra_ap_hir::MacroCallId;
use ra_ap_hir::Semantics;
use ra_ap_hir_expand::FileRange;
use ra_ap_hir_expand::Lookup;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::TextRange;
use ra_ap_syntax::ast::HasName;

use heisenberg_module_graph::folded_finding::FoldedFinding;
use heisenberg_module_graph::folded_finding_kind::FoldedFindingKind;
use heisenberg_module_graph::source_location::SourceLocation;
use heisenberg_source_locator::source_locator::SourceLocator;

use crate::macro_expansion::MacroExpansion;
use crate::module_dependencies_error::ModuleDependenciesError;

pub struct ExpansionChain<'chain, 'database> {
    pub call: MacroCallId,
    pub locator: &'chain SourceLocator<'chain>,
    pub semantics: &'chain Semantics<'database, RootDatabase>,
}

impl ExpansionChain<'_, '_> {
    pub fn findings(&self) -> Result<Vec<FoldedFinding>, ModuleDependenciesError> {
        let database = self.semantics.db;
        let expansion = MacroExpansion {
            call: self.call,
            database,
        };
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
                    .map(|name| {
                        self.locate(match location.kind.file_id() {
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
                        })
                        .map(|located| FoldedFinding {
                            kind: FoldedFindingKind::MacroExpansion {
                                depth,
                                name: name.text().to_owned(),
                            },
                            location: located,
                        })
                    })
            })
            .chain(expansion.syntax_errors().iter().map(|error| {
                self.located_in_expansion(error.range())
                    .map(|location| FoldedFinding {
                        kind: FoldedFindingKind::SyntaxError {
                            message: error.to_string(),
                        },
                        location,
                    })
            }))
            .chain(expansion.unreadable_literals().into_iter().map(|literal| {
                self.located_in_expansion(literal.text_range())
                    .map(|location| FoldedFinding {
                        kind: FoldedFindingKind::UnreadableLiteral {
                            text: literal.text().to_owned(),
                        },
                        location,
                    })
            }))
            .collect()
    }

    fn locate(
        &self,
        FileRange { file_id, range }: FileRange,
    ) -> Result<SourceLocation, ModuleDependenciesError> {
        self.locator.locate(FileRangeWrapper {
            file_id: file_id.file_id(self.semantics.db),
            range,
        })
    }

    fn located_in_expansion(
        &self,
        range: TextRange,
    ) -> Result<SourceLocation, ModuleDependenciesError> {
        self.locate(
            InFile::new(HirFileId::from(self.call), range)
                .original_node_file_range(self.semantics.db)
                .0,
        )
    }
}
