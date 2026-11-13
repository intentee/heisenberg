use proc_macro::Delimiter;
use proc_macro::Group;
use proc_macro::Ident;
use proc_macro::LexError;
use proc_macro::Literal;
use proc_macro::Punct;
use proc_macro::Spacing;
use proc_macro::Span;
use proc_macro::TokenStream;
use proc_macro::TokenTree;

fn compile_error(error: &LexError) -> TokenStream {
    [
        TokenTree::Ident(Ident::new("compile_error", Span::call_site())),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::from(TokenTree::Literal(Literal::string(&error.to_string()))),
        )),
    ]
    .into_iter()
    .collect()
}

fn registration(name: TokenTree) -> TokenStream {
    let call: TokenStream = "registry::submit!"
        .parse()
        .unwrap_or_else(|error| compile_error(&error));

    call.into_iter()
        .chain([
            TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::from(name))),
            TokenTree::Punct(Punct::new(';', Spacing::Alone)),
        ])
        .collect()
}

#[proc_macro_attribute]
pub fn registered(_arguments: TokenStream, item: TokenStream) -> TokenStream {
    let mut tokens = item.clone().into_iter();

    tokens
        .by_ref()
        .find(|token| matches!(token, TokenTree::Ident(keyword) if keyword.to_string() == "fn"));

    item.into_iter()
        .chain(tokens.next().map(registration).unwrap_or_default())
        .collect()
}
