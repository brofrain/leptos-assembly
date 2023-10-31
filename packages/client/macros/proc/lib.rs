use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

mod app_item_fn_wrapper;

mod app_component;
use app_component::AppComponent;

mod app_server_fn;
use app_server_fn::AppServerFn;

#[proc_macro_attribute]
pub fn component(_: TokenStream, t: TokenStream) -> TokenStream {
    TokenStream::from(parse_macro_input!(t as AppComponent).into_token_stream())
}

#[proc_macro_attribute]
pub fn server(_: TokenStream, t: TokenStream) -> TokenStream {
    TokenStream::from(parse_macro_input!(t as AppServerFn).into_token_stream())
}
