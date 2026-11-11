use ra_ap_hir::ExpandResult;
use ra_ap_hir::FileRangeWrapper;
use ra_ap_hir::HirFileId;
use ra_ap_hir::HirFilePosition;
use ra_ap_hir::InFile;
use ra_ap_hir_expand::Lookup;
use ra_ap_hir_expand::MacroCallId;
use ra_ap_hir_expand::MacroCallLoc;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::NodeOrToken;
use ra_ap_syntax::PreorderWithTokens;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::SyntaxToken;
use ra_ap_syntax::TextRange;
use ra_ap_syntax::WalkEvent;
use ra_ap_syntax::ast;
use ra_ap_syntax::ast::HasName;

use heisenberg_error_context::contextual_error::ContextualError;
use heisenberg_expanded_code::expansion_syntax_errors::expansion_syntax_errors;
use heisenberg_module_graph::source_location::SourceLocation;
use heisenberg_source_locator::written_range::written_range;

use crate::analysis_context::AnalysisContext;
use crate::code_activity::CodeActivity;
use crate::coverage_measurement_error::CoverageMeasurementError;
use crate::enclosing_function::EnclosingFunction;
use crate::function_node::function_node;
use crate::measured_file::MeasuredFile;
use crate::measured_functions::MeasuredFunctions;
use crate::parsed_file::ParsedFile;

pub struct FunctionCollection<'collection, 'database> {
    pub context: &'collection AnalysisContext<'collection, 'database>,
    pub file: &'collection MeasuredFile<'collection>,
}

impl FunctionCollection<'_, '_> {
    pub fn collect(&self) -> Result<MeasuredFunctions, CoverageMeasurementError> {
        let ParsedFile { file_id, root } = self.file.parsed(self.context.test_modules.semantics);
        let mut functions = MeasuredFunctions::default();

        self.tree(file_id, &root, &mut Vec::new(), &mut functions)
            .map(|()| functions)
    }

    fn analysed_node(
        &self,
        file_id: HirFileId,
        node: &SyntaxNode,
        preorder: &mut PreorderWithTokens,
        enclosing: &mut Vec<EnclosingFunction>,
        functions: &mut MeasuredFunctions,
    ) -> Result<(), CoverageMeasurementError> {
        if ast::Attr::can_cast(node.kind())
            || ast::MacroRules::can_cast(node.kind())
            || ast::MacroDef::can_cast(node.kind())
            || ast::TokenTree::can_cast(node.kind())
        {
            preorder.skip_subtree();

            Ok(())
        } else if let Some(ExpandResult {
            value:
                InFile {
                    file_id: expansion_file,
                    value,
                },
            err,
        }) = ast::Item::cast(node.clone())
            .and_then(|item| self.context.test_modules.semantics.expand_attr_macro(&item))
        {
            preorder.skip_subtree();

            expansion_file
                .macro_file()
                .ok_or(CoverageMeasurementError::AttributeExpansionWithoutMacroCall(expansion_file))
                .and_then(|call| {
                    self.expansion(call, ExpandResult { value, err }, enclosing, functions)
                })
                .and_then(|()| self.rewritten_functions(file_id, node, functions))
        } else if let Some(function) = ast::Fn::cast(node.clone()) {
            self.function(file_id, &function, enclosing, functions)
        } else if let Some(call) = ast::MacroCall::cast(node.clone()) {
            self.call(file_id, &call, enclosing, functions)
        } else {
            Ok(())
        }
    }

    fn call(
        &self,
        file_id: HirFileId,
        call: &ast::MacroCall,
        enclosing: &mut Vec<EnclosingFunction>,
        functions: &mut MeasuredFunctions,
    ) -> Result<(), CoverageMeasurementError> {
        let semantics = self.context.test_modules.semantics;

        semantics.to_def(call).map_or_else(
            || {
                let position = HirFilePosition {
                    file_id,
                    offset: call.syntax().text_range().start(),
                };

                call.path()
                    .ok_or(CoverageMeasurementError::MacroCallWithoutPath(position))
                    .and_then(|path| self.location(file_id, path.syntax().text_range()))
                    .and_then(|call| Err(CoverageMeasurementError::UnresolvedMacroCall { call }))
            },
            |expansion| {
                self.expansion(expansion, semantics.expand(expansion), enclosing, functions)
            },
        )
    }

    fn expansion(
        &self,
        call: MacroCallId,
        ExpandResult { value: root, err }: ExpandResult<SyntaxNode>,
        enclosing: &mut Vec<EnclosingFunction>,
        functions: &mut MeasuredFunctions,
    ) -> Result<(), CoverageMeasurementError> {
        let database = self.context.test_modules.semantics.db;

        err.map_or(Ok(()), |error| {
            self.invocation(call).and_then(|location| {
                Err(CoverageMeasurementError::FailedMacroExpansion {
                    call: location,
                    message: error.render_to_string(database).message,
                    error,
                })
            })
        })
        .and_then(|()| {
            expansion_syntax_errors(database, call)
                .first()
                .cloned()
                .map_or(Ok(()), |error| {
                    self.invocation(call).and_then(|location| {
                        Err(CoverageMeasurementError::UnparsableMacroExpansion(
                            ContextualError {
                                context: location,
                                source: error,
                            },
                        ))
                    })
                })
        })
        .and_then(|()| self.tree(HirFileId::from(call), &root, enclosing, functions))
    }

    fn function(
        &self,
        file_id: HirFileId,
        function: &ast::Fn,
        enclosing: &mut Vec<EnclosingFunction>,
        functions: &mut MeasuredFunctions,
    ) -> Result<(), CoverageMeasurementError> {
        self.context
            .identity
            .body_brace(file_id, function)
            .and_then(|brace| {
                brace.map_or(Ok(()), |brace| {
                    let offset = function.syntax().text_range().start();

                    function
                        .name()
                        .ok_or(CoverageMeasurementError::FunctionWithoutName { file_id, offset })
                        .and_then(|name| self.location(file_id, name.syntax().text_range()))
                        .map(|location| {
                            functions.register(brace, location);
                            functions
                                .braces
                                .insert(function_node(file_id, function), brace);
                            enclosing.push(EnclosingFunction {
                                brace,
                                node: function.syntax().clone(),
                            });
                        })
                })
            })
    }

    fn invocation(&self, call: MacroCallId) -> Result<SourceLocation, CoverageMeasurementError> {
        let database = self.context.test_modules.semantics.db;
        let MacroCallLoc { kind, krate, .. } = call.lookup(database);
        let FileRangeWrapper { file_id, range } = kind.original_call_range(database, *krate);

        self.context.locator.locate(FileRangeWrapper {
            file_id: file_id.file_id(database),
            range,
        })
    }

    fn location(
        &self,
        file_id: HirFileId,
        range: TextRange,
    ) -> Result<SourceLocation, CoverageMeasurementError> {
        self.context.locator.locate(written_range(
            self.context.test_modules.semantics.db,
            file_id,
            range,
        ))
    }

    fn node(
        &self,
        file_id: HirFileId,
        node: &SyntaxNode,
        activity: &CodeActivity<'_>,
        preorder: &mut PreorderWithTokens,
        enclosing: &mut Vec<EnclosingFunction>,
        functions: &mut MeasuredFunctions,
    ) -> Result<(), CoverageMeasurementError> {
        if self.context.test_modules.contains(node) || !activity.is_active(node) {
            preorder.skip_subtree();

            Ok(())
        } else if activity.is_production_only(node) {
            self.location(file_id, node.text_range())
                .and_then(|code| Err(CoverageMeasurementError::ProductionOnlyCode { code }))
        } else {
            self.production_only_attributes(file_id, node, activity)
                .and_then(|()| self.analysed_node(file_id, node, preorder, enclosing, functions))
        }
    }

    fn production_only_attributes(
        &self,
        file_id: HirFileId,
        node: &SyntaxNode,
        activity: &CodeActivity<'_>,
    ) -> Result<(), CoverageMeasurementError> {
        activity
            .first_production_only_meta(node)
            .map_or(Ok(()), |meta| {
                self.location(file_id, meta.syntax().text_range())
                    .and_then(|attribute| {
                        Err(CoverageMeasurementError::ProductionOnlyAttribute { attribute })
                    })
            })
    }

    fn rewritten_functions(
        &self,
        file_id: HirFileId,
        item: &SyntaxNode,
        MeasuredFunctions { braces, names, .. }: &mut MeasuredFunctions,
    ) -> Result<(), CoverageMeasurementError> {
        item.descendants()
            .filter_map(ast::Fn::cast)
            .try_for_each(|function| {
                self.context
                    .identity
                    .body_brace(file_id, &function)
                    .map(|brace| {
                        if let Some(brace) = brace.filter(|brace| names.contains_key(brace)) {
                            braces.insert(function_node(file_id, &function), brace);
                        }
                    })
            })
    }

    fn token(
        &self,
        file_id: HirFileId,
        token: &SyntaxToken,
        enclosing: &[EnclosingFunction],
        functions: &mut MeasuredFunctions,
    ) {
        if file_id.is_macro()
            && !token.kind().is_trivia()
            && let Some(position) = self.context.identity.measured_position(file_id, token)
        {
            functions.claim(
                position,
                enclosing
                    .last()
                    .map(|EnclosingFunction { brace, .. }| *brace),
            );
        }
    }

    fn tree(
        &self,
        file_id: HirFileId,
        root: &SyntaxNode,
        enclosing: &mut Vec<EnclosingFunction>,
        functions: &mut MeasuredFunctions,
    ) -> Result<(), CoverageMeasurementError> {
        let activity = CodeActivity::new(self.context, file_id, self.file.krate);
        let mut preorder = root.preorder_with_tokens();
        let mut walked = Ok(());

        while walked.is_ok()
            && let Some(event) = preorder.next()
        {
            walked = match event {
                WalkEvent::Enter(NodeOrToken::Node(node)) => self.node(
                    file_id,
                    &node,
                    &activity,
                    &mut preorder,
                    enclosing,
                    functions,
                ),
                WalkEvent::Enter(NodeOrToken::Token(token)) => {
                    self.token(file_id, &token, enclosing, functions);

                    Ok(())
                }
                WalkEvent::Leave(NodeOrToken::Node(node)) => {
                    if enclosing
                        .last()
                        .is_some_and(|EnclosingFunction { node: function, .. }| *function == node)
                    {
                        enclosing.pop();
                    }

                    Ok(())
                }
                WalkEvent::Leave(NodeOrToken::Token(_)) => Ok(()),
            };
        }

        walked
    }
}
