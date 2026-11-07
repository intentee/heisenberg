use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn service(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    passthrough(item)
}

#[proc_macro_attribute]
pub fn singleton(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    passthrough(item)
}

fn passthrough(item: TokenStream) -> TokenStream {
    item
}
