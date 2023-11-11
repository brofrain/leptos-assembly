use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

mod item_fn_wrapper;

mod component;
use component::Component;

mod server_fn;
use server_fn::ServerFn;

#[proc_macro_attribute]
pub fn component(_: TokenStream, t: TokenStream) -> TokenStream {
    TokenStream::from(parse_macro_input!(t as Component).into_token_stream())
}

#[proc_macro_attribute]
pub fn server(_: TokenStream, t: TokenStream) -> TokenStream {
    TokenStream::from(parse_macro_input!(t as ServerFn).into_token_stream())
}
