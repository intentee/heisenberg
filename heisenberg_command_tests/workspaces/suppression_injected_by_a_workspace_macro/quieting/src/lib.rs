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

#[proc_macro_attribute]
pub fn quiet(_arguments: TokenStream, item: TokenStream) -> TokenStream {
    let allowance: TokenStream = "#[allow(dead_code)]"
        .parse()
        .unwrap_or_else(|error| compile_error(&error));

    allowance.into_iter().chain(item).collect()
}
