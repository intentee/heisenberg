use std::iter;

use ra_ap_hir::FieldSource;
use ra_ap_hir::FileRange;
use ra_ap_hir::HirFileId;
use ra_ap_hir::InFile;
use ra_ap_hir::Semantics;
use ra_ap_hir_expand::Lookup;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::defs::Definition;
use ra_ap_ide_db::defs::IdentClass;
use ra_ap_ide_db::defs::NameRefClass;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::AstToken;
use ra_ap_syntax::NodeOrToken;
use ra_ap_syntax::SyntaxElement;
use ra_ap_syntax::SyntaxToken;
use ra_ap_syntax::TextRange;
use ra_ap_syntax::TextSize;
use ra_ap_syntax::ast;

use heisenberg_expanded_code::code_tokens::code_tokens;
use heisenberg_module_graph::definition_identity::DefinitionIdentity;
use heisenberg_module_graph::fingerprint_token::FingerprintToken;
use heisenberg_module_graph::source_position::SourcePosition;
use heisenberg_module_graph::token_spelling::TokenSpelling;
use heisenberg_module_graph::written_offset::WrittenOffset;
use heisenberg_source_locator::original_file_id::original_file_id;
use heisenberg_source_locator::source_locator::SourceLocator;

use crate::module_dependencies_error::ModuleDependenciesError;
use crate::template_captures::TemplateCaptures;

fn shorthand_path(token: &SyntaxToken) -> Option<ast::Path> {
    token
        .parent()
        .and_then(ast::NameRef::cast)
        .and_then(|name_ref| ast::RecordExprField::for_field_name(&name_ref))
        .filter(|field| field.name_ref().is_none())
        .and_then(|field| field.expr())
        .and_then(|value| ast::PathExpr::cast(value.syntax().clone()))
        .and_then(|value| value.path())
}

fn spelling(token: &SyntaxToken) -> TokenSpelling {
    TokenSpelling {
        kind: u16::from(token.kind()),
        text: token.text().to_owned(),
    }
}

pub struct FunctionFingerprint<'fingerprint, 'database> {
    pub function: &'fingerprint ast::Fn,
    pub locator: &'fingerprint SourceLocator<'fingerprint>,
    pub semantics: &'fingerprint Semantics<'database, RootDatabase>,
    pub template_captures: &'fingerprint TemplateCaptures<'fingerprint, 'database>,
}

impl<'database> FunctionFingerprint<'_, 'database> {
    pub fn tokens(&self) -> Result<Vec<FingerprintToken>, ModuleDependenciesError> {
        let mut declared: Vec<Definition<'database>> = Vec::new();

        self.function
            .syntax()
            .children_with_tokens()
            .filter(|element| match element {
                NodeOrToken::Node(node) => {
                    !ast::Attr::can_cast(node.kind())
                        && !ast::Name::can_cast(node.kind())
                        && !ast::Visibility::can_cast(node.kind())
                }
                NodeOrToken::Token(token) => !ast::AnyComment::can_cast(token.kind()),
            })
            .flat_map(|element| {
                code_tokens(
                    match element {
                        NodeOrToken::Node(node) => node
                            .descendants_with_tokens()
                            .collect::<Vec<SyntaxElement>>(),
                        NodeOrToken::Token(token) => vec![NodeOrToken::Token(token)],
                    }
                    .into_iter(),
                )
            })
            .map(|token| self.token(&token, &mut declared))
            .collect()
    }

    fn declaration_index(
        definition: Definition<'database>,
        declared: &mut Vec<Definition<'database>>,
    ) -> DefinitionIdentity {
        let index = declared
            .iter()
            .position(|seen| *seen == definition)
            .unwrap_or(declared.len());

        if index == declared.len() {
            declared.push(definition);
        }

        DefinitionIdentity::Declared { index }
    }

    fn declaration_range(&self, definition: Definition<'database>) -> Option<FileRange> {
        match definition {
            Definition::Field(field) => self.semantics.source(field).map(|source| {
                let node = match &source.value {
                    FieldSource::Named(named) => named.syntax().clone(),
                    FieldSource::Pos(positional) => positional.syntax().clone(),
                };

                source
                    .with_value(&node)
                    .original_file_range_rooted(self.semantics.db)
            }),
            other => other.range_for_rename(self.semantics),
        }
    }

    fn denoted(&self, token: &SyntaxToken) -> Vec<Definition<'database>> {
        IdentClass::classify_token(self.semantics, token).map_or_else(Vec::new, |class| {
            let is_field = matches!(
                class,
                IdentClass::NameRefClass(NameRefClass::Definition(Definition::Field(_), _))
            );
            let definitions: Vec<Definition<'database>> = class
                .definitions()
                .into_iter()
                .map(|(definition, _)| definition)
                .collect();

            match shorthand_path(token).filter(|_| is_field) {
                Some(path) => self
                    .semantics
                    .resolve_path(&path)
                    .map_or_else(Vec::new, |value| {
                        definitions
                            .into_iter()
                            .chain(iter::once(Definition::from(value)))
                            .collect()
                    }),
                None => definitions,
            }
        })
    }

    fn identity(
        &self,
        definition: Definition<'database>,
        declared: &mut Vec<Definition<'database>>,
        token: &SyntaxToken,
    ) -> Result<DefinitionIdentity, ModuleDependenciesError> {
        let database = self.semantics.db;

        match definition {
            Definition::Local(local) if !local.is_self(database) => {
                Ok(Self::declaration_index(definition, declared))
            }
            Definition::GenericParam(_)
            | Definition::InlineAsmOperand(_)
            | Definition::Label(_) => Ok(Self::declaration_index(definition, declared)),
            Definition::SelfType(implementation) => self
                .semantics
                .source(implementation)
                .ok_or(ModuleDependenciesError::SelfTypeWithoutSource(
                    implementation,
                ))
                .and_then(|InFile { file_id, value }| {
                    self.position(file_id, value.syntax().text_range().start())
                })
                .map(DefinitionIdentity::Impl),
            other => self.declaration_range(other).map_or_else(
                || self.unplaced(other, declared, token),
                |range| self.written(range).map(DefinitionIdentity::Written),
            ),
        }
    }

    fn named(
        &self,
        token: &SyntaxToken,
        declared: &mut Vec<Definition<'database>>,
    ) -> Result<FingerprintToken, ModuleDependenciesError> {
        let candidates: Vec<SyntaxToken> = if token
            .parent_ancestors()
            .any(|ancestor| ast::TokenTree::can_cast(ancestor.kind()))
        {
            self.semantics
                .descend_into_macros_exact(token.clone())
                .into_iter()
                .collect()
        } else {
            vec![token.clone()]
        };
        let named: Vec<SyntaxToken> = candidates
            .into_iter()
            .filter(|candidate| {
                candidate.parent().is_some_and(|parent| {
                    (ast::Name::can_cast(parent.kind())
                        || ast::NameRef::can_cast(parent.kind())
                        || ast::Lifetime::can_cast(parent.kind()))
                        && !parent
                            .parent()
                            .is_some_and(|owner| ast::FormatArgsArg::can_cast(owner.kind()))
                })
            })
            .collect();
        let definitions: Vec<Definition<'database>> = named
            .iter()
            .flat_map(|candidate| self.denoted(candidate))
            .collect();

        if !definitions.is_empty() {
            definitions
                .into_iter()
                .map(|definition| self.identity(definition, declared, token))
                .collect::<Result<Vec<DefinitionIdentity>, ModuleDependenciesError>>()
                .map(FingerprintToken::Definitions)
        } else if named.is_empty() {
            Ok(FingerprintToken::Spelling(spelling(token)))
        } else {
            self.unresolved(token)
        }
    }

    fn position(
        &self,
        file_id: HirFileId,
        offset: TextSize,
    ) -> Result<SourcePosition, ModuleDependenciesError> {
        let database = self.semantics.db;

        match file_id {
            HirFileId::FileId(file) => self
                .written(FileRange {
                    file_id: file,
                    range: TextRange::empty(offset),
                })
                .map(SourcePosition::Written),
            HirFileId::MacroFile(call) => {
                let location = call.lookup(database);
                let InFile {
                    file_id: node_file,
                    value: node,
                } = location.to_node(database);

                self.position(node_file, node.text_range().start())
                    .and_then(|node| {
                        self.written(location.kind.original_call_range(database, location.krate))
                            .map(|call| SourcePosition::Expanded {
                                call,
                                call_node: Box::new(node),
                                offset: u32::from(offset),
                            })
                    })
            }
        }
    }

    fn template(
        &self,
        token: &SyntaxToken,
        captures: Vec<Definition<'database>>,
        declared: &mut Vec<Definition<'database>>,
    ) -> Result<FingerprintToken, ModuleDependenciesError> {
        captures
            .into_iter()
            .map(|definition| self.identity(definition, declared, token))
            .collect::<Result<Vec<DefinitionIdentity>, ModuleDependenciesError>>()
            .map(|captures| FingerprintToken::FormatTemplate {
                captures,
                spelling: spelling(token),
            })
    }

    fn token(
        &self,
        token: &SyntaxToken,
        declared: &mut Vec<Definition<'database>>,
    ) -> Result<FingerprintToken, ModuleDependenciesError> {
        if ast::String::can_cast(token.kind()) {
            let captures = self.template_captures.captures(token);

            if captures.is_empty() {
                Ok(FingerprintToken::Spelling(spelling(token)))
            } else {
                captures
                    .into_iter()
                    .collect::<Option<Vec<Definition<'database>>>>()
                    .map_or_else(
                        || self.unresolved(token),
                        |definitions| self.template(token, definitions, declared),
                    )
            }
        } else {
            self.named(token, declared)
        }
    }

    fn unplaced(
        &self,
        definition: Definition<'database>,
        declared: &mut Vec<Definition<'database>>,
        token: &SyntaxToken,
    ) -> Result<DefinitionIdentity, ModuleDependenciesError> {
        let database = self.semantics.db;
        let name = definition
            .name(database)
            .map(|name| name.as_str().to_owned());

        match definition.enclosing_definition(database) {
            Some(enclosing) => self.identity(enclosing, declared, token).map(|container| {
                DefinitionIdentity::Member {
                    container: Box::new(container),
                    name,
                }
            }),
            None => match definition.krate(database) {
                Some(krate) => self
                    .locator
                    .file(krate.root_file(database))
                    .map(|crate_root| DefinitionIdentity::CrateLevel {
                        crate_root,
                        name,
                        spelling: spelling(token),
                    }),
                None => Ok(DefinitionIdentity::Builtin {
                    name,
                    spelling: spelling(token),
                }),
            },
        }
    }

    fn unresolved(&self, token: &SyntaxToken) -> Result<FingerprintToken, ModuleDependenciesError> {
        self.locator
            .file(original_file_id(
                self.semantics.db,
                self.semantics.hir_file_for(self.function.syntax()),
            ))
            .map(|file| {
                FingerprintToken::Unresolved(WrittenOffset {
                    file,
                    offset: u32::from(token.text_range().start()),
                })
            })
    }

    fn written(
        &self,
        FileRange { file_id, range }: FileRange,
    ) -> Result<WrittenOffset, ModuleDependenciesError> {
        self.locator
            .file(file_id.file_id(self.semantics.db))
            .map(|file| WrittenOffset {
                file,
                offset: u32::from(range.start()),
            })
    }
}
