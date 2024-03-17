#![feature(lazy_cell)]
#![feature(proc_macro_expand)]
#![feature(let_chains)]
#![feature(track_path)]

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

mod test_selectors;
use test_selectors::SelectorPath;

#[proc_macro]
pub fn pin_test_selector(tokens: TokenStream) -> TokenStream {
    let selector = parse_macro_input!(tokens as SelectorPath);
    TokenStream::from(selector.into_token_stream())
}

#[proc_macro]

pub fn generate_test_selectors(_tokens: TokenStream) -> TokenStream {
    test_selectors::generate()
}
