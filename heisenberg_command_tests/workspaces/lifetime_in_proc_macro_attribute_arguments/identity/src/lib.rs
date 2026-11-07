use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn emit(arguments: TokenStream, item: TokenStream) -> TokenStream {
    let mut expanded = item;

    expanded.extend(arguments);

    expanded
}
