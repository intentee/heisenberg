use proc_macro::Delimiter;
use proc_macro::Group;
use proc_macro::Ident;
use proc_macro::Punct;
use proc_macro::Spacing;
use proc_macro::Span;
use proc_macro::TokenStream;
use proc_macro::TokenTree;

#[proc_macro_attribute]
pub fn submitted(_attribute: TokenStream, item: TokenStream) -> TokenStream {
    let mut output = item;

    output.extend([
        TokenTree::Ident(Ident::new("submit", Span::call_site())),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
        TokenTree::Punct(Punct::new(';', Spacing::Alone)),
    ]);

    output
}
