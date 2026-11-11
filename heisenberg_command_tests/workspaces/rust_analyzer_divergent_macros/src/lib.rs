use std::env;

use proc_macro::Ident;
use proc_macro::Span;
use proc_macro::TokenStream;
use proc_macro::TokenTree;

const RUST_ANALYZER_PROC_MACRO_SERVER_VARIABLE: &str = "RUST_ANALYZER_INTERNALS_DO_NOT_USE";

fn expanded_by_rust_analyzer() -> bool {
    env::var_os(RUST_ANALYZER_PROC_MACRO_SERVER_VARIABLE).is_some()
}

#[proc_macro]
pub fn failing_in_rust_analyzer(item: TokenStream) -> TokenStream {
    assert!(
        !expanded_by_rust_analyzer(),
        "rust-analyzer expands this macro"
    );
    item
}

#[proc_macro]
pub fn hidden_from_rust_analyzer(item: TokenStream) -> TokenStream {
    if expanded_by_rust_analyzer() {
        TokenStream::new()
    } else {
        item
    }
}

#[proc_macro]
pub fn unparsable_in_rust_analyzer(item: TokenStream) -> TokenStream {
    if expanded_by_rust_analyzer() {
        TokenStream::from(TokenTree::Ident(Ident::new("fn", Span::call_site())))
    } else {
        item
    }
}

#[proc_macro_attribute]
pub fn failing_attribute_in_rust_analyzer(
    _attribute: TokenStream,
    item: TokenStream,
) -> TokenStream {
    assert!(
        !expanded_by_rust_analyzer(),
        "rust-analyzer expands this attribute"
    );
    item
}

#[proc_macro_attribute]
pub fn unparsable_attribute_in_rust_analyzer(
    _attribute: TokenStream,
    item: TokenStream,
) -> TokenStream {
    if expanded_by_rust_analyzer() {
        TokenStream::from(TokenTree::Ident(Ident::new("fn", Span::call_site())))
    } else {
        item
    }
}
