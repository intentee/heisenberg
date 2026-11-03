use proc_macro::Group;
use proc_macro::Span;
use proc_macro::TokenStream;
use proc_macro::TokenTree;

fn respanned(stream: TokenStream, span: Span) -> TokenStream {
    stream
        .into_iter()
        .map(|token| match token {
            TokenTree::Group(group) => {
                let mut respanned_group = Group::new(group.delimiter(), respanned(group.stream(), span));

                respanned_group.set_span(span);

                TokenTree::Group(respanned_group)
            }
            mut other => {
                other.set_span(span);

                other
            }
        })
        .collect()
}

#[proc_macro_attribute]
pub fn generate(_attribute: TokenStream, item: TokenStream) -> TokenStream {
    let input_span = item.clone().into_iter().map(|token| token.span()).next();
    let mut expanded = item;

    expanded.extend(
        "#[allow(dead_code)] fn generated_allowed() {} #[cfg(test)] fn generated_test_only() {}"
            .parse::<TokenStream>()
            .into_iter()
            .flatten(),
    );
    expanded.extend(input_span.into_iter().flat_map(|span| {
        "#[allow(dead_code)] fn generated_with_an_input_span() {}"
            .parse::<TokenStream>()
            .into_iter()
            .flat_map(move |stream| respanned(stream, span))
    }));

    expanded
}
