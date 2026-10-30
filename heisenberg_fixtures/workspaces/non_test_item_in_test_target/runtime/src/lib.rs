use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn test(_attribute: TokenStream, item: TokenStream) -> TokenStream {
    item
}
