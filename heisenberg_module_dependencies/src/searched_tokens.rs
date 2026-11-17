use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;

use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::NodeOrToken;
use ra_ap_syntax::SyntaxKind;
use ra_ap_vfs::FileId;

use crate::string_site::StringSite;
use crate::token_site::TokenSite;

#[derive(Default)]
pub struct SearchedTokens {
    pub names: HashMap<String, Vec<TokenSite>>,
    pub strings: BTreeMap<FileId, Vec<StringSite>>,
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
                let kind = token.kind();
                let offset = token.text_range().start();

                if kind == SyntaxKind::STRING {
                    tokens.strings.entry(*file).or_default().push(StringSite {
                        offset,
                        text: token.text().to_owned(),
                    });
                } else if matches!(
                    kind,
                    SyntaxKind::IDENT
                        | SyntaxKind::INT_NUMBER
                        | SyntaxKind::SELF_KW
                        | SyntaxKind::SELF_TYPE_KW
                ) {
                    tokens
                        .names
                        .entry(token.text().to_owned())
                        .or_default()
                        .push(TokenSite {
                            file: *file,
                            offset,
                        });
                }
            }
        }

        tokens
    }
}
