#![feature(lazy_cell)]
#![feature(proc_macro_expand)]
#![feature(let_chains)]

use common::vendor::serde_json;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, LitStr};

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

use std::{
    fs::File,
    io::{BufWriter, Write},
};

#[proc_macro]
pub fn generate_test_selectors_json(_tokens: TokenStream) -> TokenStream {
    let crate_name = syn::parse::<LitStr>({
        let env_crate_name: TokenStream =
            quote! { std::env!("CARGO_CRATE_NAME") }.into();
        env_crate_name
            .expand_expr()
            .expect("crate name should expand")
    })
    .expect("crate name should be a string literal")
    .value();

    let file = File::create(format!("target/{crate_name}_test_selectors.json"))
        .expect("JSON file should be created");

    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &String::new()).unwrap();
    writer.flush().unwrap();

    TokenStream::new()
}
