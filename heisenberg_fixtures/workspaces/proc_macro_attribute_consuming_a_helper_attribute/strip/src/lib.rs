use proc_macro::Delimiter;
use proc_macro::TokenStream;
use proc_macro::TokenTree;

#[proc_macro_attribute]
pub fn table(_attribute: TokenStream, item: TokenStream) -> TokenStream {
    let mut tokens = item.into_iter().peekable();
    let mut output = TokenStream::new();

    while let Some(token) = tokens.next() {
        let starts_attribute = matches!(&token, TokenTree::Punct(punct) if punct.as_char() == '#')
            && matches!(tokens.peek(), Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Bracket);

        if starts_attribute {
            tokens.next();
        } else {
            output.extend([token]);
        }
    }

    output
}
