use proc_macro::Delimiter;
use proc_macro::Group;
use proc_macro::Ident;
use proc_macro::TokenStream;
use proc_macro::TokenTree;

fn after_keyword(input: &TokenStream, keyword: &str) -> Option<TokenTree> {
    input
        .clone()
        .into_iter()
        .skip_while(|token| !matches!(token, TokenTree::Ident(word) if word.to_string() == keyword))
        .nth(1)
}

fn helper_argument(input: &TokenStream) -> Option<TokenTree> {
    input.clone().into_iter().find_map(|token| match token {
        TokenTree::Group(attribute) if attribute.delimiter() == Delimiter::Bracket => {
            let mut inner = attribute.stream().into_iter();

            match (inner.next(), inner.next()) {
                (Some(TokenTree::Ident(name)), Some(TokenTree::Group(arguments)))
                    if name.to_string() == "default_with" =>
                {
                    arguments.stream().into_iter().next()
                }
                _ => None,
            }
        }
        _ => None,
    })
}

fn tokens(text: &str) -> Vec<TokenTree> {
    text.parse::<TokenStream>().into_iter().flatten().collect()
}

#[proc_macro_derive(Parse, attributes(default_with))]
pub fn parse(input: TokenStream) -> TokenStream {
    let name = after_keyword(&input, "struct");
    let port = helper_argument(&input);

    name.into_iter()
        .flat_map(|name| {
            let constructor: TokenStream = tokens("pub fn parse() -> Self")
                .into_iter()
                .chain([TokenTree::Group(Group::new(
                    Delimiter::Brace,
                    TokenStream::from(name.clone()),
                ))])
                .collect();
            let configured: TokenStream = tokens("pub fn configured_port() -> u16")
                .into_iter()
                .chain([TokenTree::Group(Group::new(
                    Delimiter::Brace,
                    port.clone()
                        .into_iter()
                        .chain(tokens("()"))
                        .collect(),
                ))])
                .collect();
            let body: TokenStream = constructor.into_iter().chain(configured).collect();

            [
                TokenTree::Ident(Ident::new("impl", proc_macro::Span::call_site())),
                name,
                TokenTree::Group(Group::new(Delimiter::Brace, body)),
            ]
        })
        .collect()
}
