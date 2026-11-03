use proc_macro::Delimiter;
use proc_macro::Group;
use proc_macro::Punct;
use proc_macro::Spacing;
use proc_macro::TokenStream;
use proc_macro::TokenTree;

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
            .into_iter()
            .flatten()
            .chain([name, TokenTree::Punct(Punct::new(';', Spacing::Alone))])
            .collect();

        "#[test] fn trial()"
            .parse::<TokenStream>()
            .into_iter()
            .flatten()
            .chain([TokenTree::Group(Group::new(Delimiter::Brace, body))])
    }));

    expanded
}
