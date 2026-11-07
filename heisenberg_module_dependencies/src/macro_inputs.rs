use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use ra_ap_hir_expand::Lookup;
use ra_ap_hir_expand::MacroCallId;
use ra_ap_hir_expand::MacroCallKind;
use ra_ap_hir_expand::tt::DelimiterKind;
use ra_ap_hir_expand::tt::TtElement;
use ra_ap_hir_expand::tt::TtIter;
use ra_ap_ide_db::RootDatabase;

use crate::input_token::InputToken;
use crate::macro_input::MacroInput;

const fn closing_character(kind: DelimiterKind) -> Option<char> {
    match kind {
        DelimiterKind::Brace => Some('}'),
        DelimiterKind::Bracket => Some(']'),
        DelimiterKind::Invisible => None,
        DelimiterKind::Parenthesis => Some(')'),
    }
}

fn input_tokens(elements: TtIter<'_>) -> Vec<InputToken> {
    elements
        .flat_map(|element| match element {
            TtElement::Leaf(leaf) => vec![InputToken {
                span: *leaf.span(),
                text: leaf.to_string(),
            }],
            TtElement::Subtree(subtree, nested) => opening_character(subtree.delimiter.kind)
                .map(|character| InputToken {
                    span: subtree.delimiter.open,
                    text: character.to_string(),
                })
                .into_iter()
                .chain(input_tokens(nested))
                .chain(
                    closing_character(subtree.delimiter.kind).map(|character| InputToken {
                        span: subtree.delimiter.close,
                        text: character.to_string(),
                    }),
                )
                .collect(),
        })
        .collect()
}

const fn opening_character(kind: DelimiterKind) -> Option<char> {
    match kind {
        DelimiterKind::Brace => Some('{'),
        DelimiterKind::Bracket => Some('['),
        DelimiterKind::Invisible => None,
        DelimiterKind::Parenthesis => Some('('),
    }
}

pub struct MacroInputs<'inputs> {
    database: &'inputs RootDatabase,
    inputs: RefCell<HashMap<MacroCallId, Rc<MacroInput>>>,
}

impl<'inputs> MacroInputs<'inputs> {
    pub fn new(database: &'inputs RootDatabase) -> Self {
        Self {
            database,
            inputs: RefCell::default(),
        }
    }

    pub fn of(&self, call: MacroCallId) -> Rc<MacroInput> {
        Rc::clone(self.inputs.borrow_mut().entry(call).or_insert_with(|| {
            let kind = &call.lookup(self.database).kind;
            let (arguments, ..) = call.macro_arg_considering_derives(self.database, kind);
            let attribute_arguments = match kind {
                MacroCallKind::Attr { attr_args, .. } => attr_args.as_deref(),
                _ => None,
            };

            Rc::new(MacroInput::new(
                input_tokens(arguments.as_token_trees().iter())
                    .into_iter()
                    .chain(
                        attribute_arguments
                            .into_iter()
                            .flat_map(|attribute| input_tokens(attribute.as_token_trees().iter())),
                    )
                    .collect(),
            ))
        }))
    }
}
