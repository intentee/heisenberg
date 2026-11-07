use std::collections::HashMap;

use ra_ap_hir_expand::tt::Span;
use ra_ap_syntax::TextRange;

use crate::input_token::InputToken;

const fn start_of(span: Span) -> Span {
    Span {
        range: TextRange::empty(span.range.start()),
        ..span
    }
}

pub struct MacroInput {
    starts: HashMap<Span, Vec<usize>>,
    tokens: Vec<InputToken>,
}

impl MacroInput {
    pub fn new(tokens: Vec<InputToken>) -> Self {
        let mut starts: HashMap<Span, Vec<usize>> = HashMap::new();

        for (index, InputToken { span, .. }) in tokens.iter().enumerate() {
            starts.entry(start_of(*span)).or_default().push(index);
        }

        Self { starts, tokens }
    }

    pub fn holds(&self, span: Span, text: &str) -> bool {
        self.starts
            .get(&start_of(span))
            .is_some_and(|indices| indices.iter().any(|index| self.spells(*index, span, text)))
    }

    fn spells(&self, index: usize, Span { anchor, ctx, range }: Span, text: &str) -> bool {
        let mut spelled = String::new();

        self.tokens
            .iter()
            .skip(index)
            .take_while(|InputToken { span, .. }| {
                span.anchor == anchor && span.ctx == ctx && range.contains_range(span.range)
            })
            .any(
                |InputToken {
                     span,
                     text: token_text,
                 }| {
                    spelled.push_str(token_text);

                    spelled == text && span.range.end() == range.end()
                },
            )
    }
}
