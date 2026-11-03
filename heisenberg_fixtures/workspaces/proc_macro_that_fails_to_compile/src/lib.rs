use proc_macro::TokenStream;

#[proc_macro]
pub fn generated(_input: TokenStream) -> TokenStream {
    let _count: u8 = "text";

    TokenStream::new()
}
