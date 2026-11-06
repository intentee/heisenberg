use proc_macro::Delimiter;
use proc_macro::Group;
use proc_macro::Ident;
use proc_macro::Literal;
use proc_macro::Punct;
use proc_macro::Spacing;
use proc_macro::Span;
use proc_macro::TokenStream;
use proc_macro::TokenTree;

fn wrapped(item: TokenStream, name: &str, value: Vec<TokenTree>) -> TokenStream {
    let body: TokenStream = [
        TokenTree::Ident(Ident::new(name, Span::call_site())),
        TokenTree::Punct(Punct::new('=', Spacing::Alone)),
    ]
    .into_iter()
    .chain(value)
    .collect();
    let mut output = item;

    output.extend([
        TokenTree::Ident(Ident::new("wrap", Span::call_site())),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Group(Group::new(Delimiter::Brace, body)),
    ]);

    output
}

#[proc_macro_attribute]
pub fn single_token(_attribute: TokenStream, item: TokenStream) -> TokenStream {
    wrapped(
        item,
        "SINGLE_TOKEN",
        vec![TokenTree::Literal(Literal::i32_suffixed(-1))],
    )
}

#[proc_macro_attribute]
pub fn separate_sign(_attribute: TokenStream, item: TokenStream) -> TokenStream {
    wrapped(
        item,
        "SEPARATE_SIGN",
        vec![
            TokenTree::Punct(Punct::new('-', Spacing::Alone)),
            TokenTree::Literal(Literal::u32_unsuffixed(1)),
        ],
    )
}
