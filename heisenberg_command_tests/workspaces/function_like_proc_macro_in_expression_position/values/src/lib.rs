use proc_macro::Literal;
use proc_macro::TokenStream;
use proc_macro::TokenTree;

#[proc_macro]
pub fn one(_input: TokenStream) -> TokenStream {
    TokenStream::from(TokenTree::Literal(Literal::u8_suffixed(1)))
}
