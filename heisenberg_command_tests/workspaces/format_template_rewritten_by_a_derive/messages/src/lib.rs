use proc_macro::Delimiter;
use proc_macro::Group;
use proc_macro::TokenStream;
use proc_macro::TokenTree;

fn after_keyword(input: &TokenStream, keyword: &str) -> Option<TokenTree> {
    input
        .clone()
        .into_iter()
        .skip_while(|token| !matches!(token, TokenTree::Ident(word) if word.to_string() == keyword))
        .nth(1)
}

fn field_name(input: &TokenStream) -> Option<TokenTree> {
    input.clone().into_iter().find_map(|token| match token {
        TokenTree::Group(fields) if fields.delimiter() == Delimiter::Brace => fields
            .stream()
            .into_iter()
            .skip_while(
                |field| !matches!(field, TokenTree::Ident(word) if word.to_string() != "pub"),
            )
            .next(),
        _ => None,
    })
}

fn tokens(text: &str) -> Vec<TokenTree> {
    text.parse::<TokenStream>().into_iter().flatten().collect()
}

fn written_template(input: &TokenStream) -> Option<TokenTree> {
    input.clone().into_iter().find_map(|token| match token {
        TokenTree::Group(attribute) if attribute.delimiter() == Delimiter::Bracket => {
            let mut inner = attribute.stream().into_iter();

            match (inner.next(), inner.next()) {
                (Some(TokenTree::Ident(name)), Some(TokenTree::Group(arguments)))
                    if name.to_string() == "message" =>
                {
                    arguments.stream().into_iter().next()
                }
                _ => None,
            }
        }
        _ => None,
    })
}

#[proc_macro_derive(Message, attributes(message))]
pub fn message(input: TokenStream) -> TokenStream {
    after_keyword(&input, "struct")
        .and_then(|name| {
            field_name(&input).and_then(|field| {
                written_template(&input).map(|written| {
                    let rewritten = tokens(&written.to_string().replace('{', "{field_"))
                        .into_iter()
                        .map(|mut token| {
                            token.set_span(written.span());
                            token
                        });
                    let arguments: TokenStream = tokens("formatter,")
                        .into_iter()
                        .chain(rewritten)
                        .chain(tokens(&format!(", field_{field} = self.{field}")))
                        .collect();
                    let body: TokenStream = tokens("::core::write!")
                        .into_iter()
                        .chain([TokenTree::Group(Group::new(Delimiter::Parenthesis, arguments))])
                        .collect();
                    let method: TokenStream = tokens(
                        "fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result",
                    )
                    .into_iter()
                    .chain([TokenTree::Group(Group::new(Delimiter::Brace, body))])
                    .collect();

                    tokens("impl ::core::fmt::Display for")
                        .into_iter()
                        .chain([name, TokenTree::Group(Group::new(Delimiter::Brace, method))])
                        .collect()
                })
            })
        })
        .unwrap_or_default()
}
