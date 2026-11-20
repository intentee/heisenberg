use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Range;

use ra_ap_hir::Name;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::syntax_helpers::format_string::FormatSpecifier;
use ra_ap_ide_db::syntax_helpers::format_string::lex_format_specifiers;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::AstToken;
use ra_ap_syntax::NodeOrToken;
use ra_ap_syntax::SyntaxKind;
use ra_ap_syntax::ast;
use ra_ap_vfs::FileId;

use crate::token_site::TokenSite;

#[derive(Default)]
pub struct SearchedTokens {
    pub captures: HashMap<Name, Vec<TokenSite>>,
    pub names: HashMap<Name, Vec<TokenSite>>,
}

impl SearchedTokens {
    pub fn new(semantics: &Semantics<'_, RootDatabase>, files: &BTreeSet<FileId>) -> Self {
        let mut tokens = Self::default();

        for file in files {
            for token in semantics
                .parse_guess_edition(*file)
                .syntax()
                .descendants_with_tokens()
                .filter_map(NodeOrToken::into_token)
            {
                let site = TokenSite {
                    file: *file,
                    offset: token.text_range().start(),
                };

                if let Some(string) = ast::String::cast(token.clone()) {
                    let mut captured = HashSet::new();

                    lex_format_specifiers(&string, &mut |range, specifier| {
                        if matches!(specifier, FormatSpecifier::Identifier)
                            && let Some(identifier) = token.text().get(Range::<usize>::from(range))
                        {
                            captured.insert(Name::new_root(identifier));
                        }
                    });

                    for name in captured {
                        tokens.captures.entry(name).or_default().push(site);
                    }
                } else if matches!(
                    token.kind(),
                    SyntaxKind::IDENT
                        | SyntaxKind::INT_NUMBER
                        | SyntaxKind::SELF_KW
                        | SyntaxKind::SELF_TYPE_KW
                ) {
                    tokens
                        .names
                        .entry(Name::new_root(token.text()))
                        .or_default()
                        .push(site);
                }
            }
        }

        tokens
    }
}
