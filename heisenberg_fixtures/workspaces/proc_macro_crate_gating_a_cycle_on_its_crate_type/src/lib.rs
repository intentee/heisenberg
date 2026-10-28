mod first;
mod second;

use proc_macro::TokenStream;

#[proc_macro]
pub fn generate(_input: TokenStream) -> TokenStream {
    second::first();
    TokenStream::new()
}
