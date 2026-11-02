use ra_ap_hir::Crate;
use ra_ap_hir::FieldSource;
use ra_ap_hir::FileRange;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::defs::Definition;
use ra_ap_ide_db::defs::IdentClass;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::AstToken;
use ra_ap_syntax::NodeOrToken;
use ra_ap_syntax::SyntaxElement;
use ra_ap_syntax::SyntaxToken;
use ra_ap_syntax::ast;

use crate::code_tokens::code_tokens;
use crate::original_file_id::original_file_id;
use crate::source_locator::SourceLocator;

pub struct FunctionFingerprint<'fingerprint, 'database> {
    pub function: &'fingerprint ast::Fn,
    pub krate: Crate,
    pub locator: &'fingerprint SourceLocator<'fingerprint>,
    pub semantics: &'fingerprint Semantics<'database, RootDatabase>,
}

impl<'database> FunctionFingerprint<'_, 'database> {
    pub fn tokens(&self) -> Vec<String> {
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
    ) -> String {
        let index = declared
            .iter()
            .position(|seen| *seen == definition)
            .unwrap_or(declared.len());

        if index == declared.len() {
            declared.push(definition);
        }

        format!("declared {index}")
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

    fn identity(
        &self,
        definition: Definition<'database>,
        declared: &mut Vec<Definition<'database>>,
    ) -> String {
        let database = self.semantics.db;

        match definition {
            Definition::Local(local) if !local.is_self(database) => {
                Self::declaration_index(definition, declared)
            }
            Definition::GenericParam(_) | Definition::Label(_) => {
                Self::declaration_index(definition, declared)
            }
            other => self
                .declaration_range(other)
                .map_or_else(|| self.label(other), |range| self.location(range)),
        }
    }

    fn label(&self, definition: Definition<'database>) -> String {
        let database = self.semantics.db;

        format!(
            "label {}",
            definition.label(database, self.krate.to_display_target(database))
        )
    }

    fn location(&self, FileRange { file_id, range }: FileRange) -> String {
        format!(
            "definition {}:{}",
            self.locator.file(file_id.file_id(self.semantics.db)),
            u32::from(range.start())
        )
    }

    fn token(&self, token: &SyntaxToken, declared: &mut Vec<Definition<'database>>) -> String {
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
                    ast::Name::can_cast(parent.kind())
                        || ast::NameRef::can_cast(parent.kind())
                        || ast::Lifetime::can_cast(parent.kind())
                })
            })
            .collect();
        let definitions: Vec<Definition<'database>> = named
            .iter()
            .filter_map(|candidate| IdentClass::classify_token(self.semantics, candidate))
            .flat_map(IdentClass::definitions)
            .map(|(definition, _)| definition)
            .collect();

        if !definitions.is_empty() {
            definitions
                .into_iter()
                .map(|definition| self.identity(definition, declared))
                .collect::<Vec<String>>()
                .join(" | ")
        } else if named.is_empty() {
            format!("{:?} {}", token.kind(), token.text())
        } else {
            format!(
                "unresolved {}:{}",
                self.locator.file(original_file_id(
                    self.semantics.db,
                    self.semantics.hir_file_for(self.function.syntax())
                )),
                u32::from(token.text_range().start())
            )
        }
    }
}
