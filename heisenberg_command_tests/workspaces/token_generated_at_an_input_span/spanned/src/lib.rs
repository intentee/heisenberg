use proc_macro::Delimiter;
use proc_macro::Group;
use proc_macro::Ident;
use proc_macro::Punct;
use proc_macro::Spacing;
use proc_macro::Span;
use proc_macro::TokenStream;
use proc_macro::TokenTree;

fn at(span: Span, mut tree: TokenTree) -> TokenTree {
    tree.set_span(span);

    tree
}

#[proc_macro_attribute]
pub fn companion(_arguments: TokenStream, item: TokenStream) -> TokenStream {
    let span = item
        .clone()
        .into_iter()
        .next()
        .map_or_else(Span::call_site, |tree| tree.span());
    let pair = TokenStream::from_iter([
        at(span, Ident::new("u8", span).into()),
        at(span, Punct::new(',', Spacing::Alone).into()),
        at(span, Ident::new("u8", span).into()),
    ]);
    let parameters = TokenStream::from_iter([
        at(span, Ident::new("x", span).into()),
        at(span, Punct::new(':', Spacing::Alone).into()),
        at(span, Group::new(Delimiter::Parenthesis, pair).into()),
    ]);
    let mut expanded = item;

    expanded.extend([
        at(span, Ident::new("fn", span).into()),
        at(span, Ident::new("companion", span).into()),
        at(span, Group::new(Delimiter::Parenthesis, parameters).into()),
        at(span, Group::new(Delimiter::Brace, TokenStream::new()).into()),
    ]);

    expanded
}
