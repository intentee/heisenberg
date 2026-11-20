use ra_ap_hir::FileRangeWrapper;
use ra_ap_hir::HirFileId;
use ra_ap_ide_db::FilePosition;
use ra_ap_ide_db::line_index;
use ra_ap_ide_db::line_index::LineCol;
use ra_ap_ide_db::line_index::LineIndex;
use ra_ap_paths::AbsPath;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::TextRange;
use ra_ap_syntax::TextSize;
use ra_ap_syntax::ast;

use heisenberg_coverage_export::exported_region::ExportedRegion;
use heisenberg_coverage_export::region_position::RegionPosition;
use heisenberg_coverage_export::region_span::RegionSpan;
use heisenberg_coverage_export::zero_based_position::ZeroBasedPosition;
use heisenberg_coverage_export::zero_based_span::ZeroBasedSpan;
use heisenberg_source_locator::item_focus_range::item_focus_range;

use crate::analysis_context::AnalysisContext;
use crate::attributed_region::AttributedRegion;
use crate::code_activity::CodeActivity;
use crate::coverage_measurement_error::CoverageMeasurementError;
use crate::function_node::function_node;
use crate::measured_file::MeasuredFile;
use crate::measured_functions::MeasuredFunctions;
use crate::measured_region::MeasuredRegion;
use crate::parsed_file::ParsedFile;
use crate::region_owner::RegionOwner;
use crate::region_start::RegionStart;

fn text_offset(
    index: &LineIndex,
    ZeroBasedPosition { line, column }: ZeroBasedPosition,
) -> Option<TextSize> {
    index
        .offset(LineCol { line, col: 0 })
        .and_then(|line_start| line_start.checked_add(TextSize::from(column)))
        .filter(|offset| index.try_line_col(*offset) == Some(LineCol { line, col: column }))
}

pub struct FileMeasurement<'measurement, 'database> {
    pub context: &'measurement AnalysisContext<'measurement, 'database>,
    pub export: &'measurement AbsPath,
    pub file: &'measurement MeasuredFile<'measurement>,
    pub measured: &'measurement MeasuredFunctions,
    pub regions: &'measurement [ExportedRegion],
}

impl FileMeasurement<'_, '_> {
    pub fn measure(&self) -> Result<Vec<AttributedRegion>, CoverageMeasurementError> {
        let semantics = self.context.test_modules.semantics;
        let ParsedFile { file_id, root } = self.file.parsed(semantics);
        let index = line_index(semantics.db, self.file.file_id);
        let activity = CodeActivity::new(self.context, file_id, self.file.krate);

        self.regions
            .iter()
            .try_fold(Vec::new(), |mut measured, region| {
                self.region(file_id, &root, index, &activity, region)
                    .map(|attributed| {
                        measured.extend(attributed);

                        measured
                    })
            })
    }

    fn enclosing_function(&self, file_id: HirFileId, node: &SyntaxNode) -> Option<FilePosition> {
        node.ancestors()
            .filter_map(ast::Fn::cast)
            .find_map(|function| {
                self.measured
                    .braces
                    .get(&function_node(file_id, &function))
                    .copied()
            })
    }

    fn holding_functions(
        &self,
        file_id: HirFileId,
        node: &SyntaxNode,
        offset: TextSize,
    ) -> Vec<FilePosition> {
        self.measured
            .claims
            .get(&FilePosition {
                file_id: self.file.file_id,
                offset,
            })
            .map_or_else(
                || self.enclosing_function(file_id, node).into_iter().collect(),
                Clone::clone,
            )
    }

    fn owner(
        &self,
        file_id: HirFileId,
        RegionStart { node, offset }: &RegionStart,
        start: RegionPosition,
    ) -> Result<RegionOwner, CoverageMeasurementError> {
        let functions = self.holding_functions(file_id, node, *offset);

        if functions.is_empty() {
            node.ancestors()
                .find(|ancestor| ast::Item::can_cast(ancestor.kind()))
                .ok_or_else(|| CoverageMeasurementError::RegionOutsideEveryItem {
                    export: self.export.to_path_buf(),
                    file: self.file.path.to_path_buf(),
                    start,
                })
                .and_then(|item| {
                    self.context.locator.locate(FileRangeWrapper {
                        file_id: self.file.file_id,
                        range: TextRange::empty(item_focus_range(&item).start()),
                    })
                })
                .map(RegionOwner::Item)
        } else {
            Ok(RegionOwner::Functions(functions))
        }
    }

    fn region(
        &self,
        file_id: HirFileId,
        root: &SyntaxNode,
        index: &LineIndex,
        activity: &CodeActivity<'_>,
        ExportedRegion {
            executed,
            exported,
            span,
        }: &ExportedRegion,
    ) -> Result<Option<AttributedRegion>, CoverageMeasurementError> {
        let MeasuredFile {
            file_id: source_file,
            path,
            ..
        } = self.file;
        let start = self.region_start(root, index, *span, *exported)?;
        let RegionStart { node, offset } = &start;

        if self.context.test_modules.contains(node) {
            Ok(None)
        } else if activity.is_active(node) {
            self.context
                .locator
                .locate(FileRangeWrapper {
                    file_id: *source_file,
                    range: TextRange::empty(*offset),
                })
                .and_then(|location| {
                    self.owner(file_id, &start, exported.start).map(|owner| {
                        Some(AttributedRegion {
                            owner,
                            region: MeasuredRegion {
                                executed: *executed,
                                location,
                            },
                        })
                    })
                })
        } else {
            Err(CoverageMeasurementError::RegionInInactiveCode {
                export: self.export.to_path_buf(),
                file: path.to_path_buf(),
                start: exported.start,
            })
        }
    }

    fn region_start(
        &self,
        root: &SyntaxNode,
        index: &LineIndex,
        ZeroBasedSpan { start, end }: ZeroBasedSpan,
        RegionSpan {
            start: exported_start,
            end: exported_end,
        }: RegionSpan,
    ) -> Result<RegionStart, CoverageMeasurementError> {
        text_offset(index, start)
            .filter(|offset| *offset < index.len() && text_offset(index, end).is_some())
            .and_then(|offset| {
                root.token_at_offset(offset)
                    .right_biased()
                    .and_then(|token| token.parent())
                    .map(|node| RegionStart { node, offset })
            })
            .ok_or_else(|| CoverageMeasurementError::RegionOutsideFileText {
                end: exported_end,
                export: self.export.to_path_buf(),
                file: self.file.path.to_path_buf(),
                start: exported_start,
            })
    }
}
