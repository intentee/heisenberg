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
pub fn trial(_attribute: TokenStream, item: TokenStream) -> TokenStream {
    let name = item
        .clone()
        .into_iter()
        .skip_while(|token| !matches!(token, TokenTree::Ident(keyword) if keyword.to_string() == "fn"))
        .nth(1);
    let mut expanded = item;

    expanded.extend(name.into_iter().flat_map(|name| {
        let body: TokenStream = "let _ ="
            .parse::<TokenStream>()
            .unwrap_or_else(|error| compile_error(&error))
            .into_iter()
            .chain([name, TokenTree::Punct(Punct::new(';', Spacing::Alone))])
            .collect();

        "#[test] fn trial()"
            .parse::<TokenStream>()
            .unwrap_or_else(|error| compile_error(&error))
            .into_iter()
            .chain([TokenTree::Group(Group::new(Delimiter::Brace, body))])
    }));

    expanded
}
