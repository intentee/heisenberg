use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn quiet(_arguments: TokenStream, item: TokenStream) -> TokenStream {
    let allowance: TokenStream = "#[allow(dead_code)]".parse().unwrap_or_default();

    allowance.into_iter().chain(item).collect()
}
