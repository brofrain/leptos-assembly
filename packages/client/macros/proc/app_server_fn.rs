// @kw duplicate macros

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};

use crate::app_item_fn_wrapper::AppItemFnWrapper;

pub struct AppServerFn(AppItemFnWrapper);

impl Parse for AppServerFn {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(AppItemFnWrapper::parse(input)?))
    }
}

impl ToTokens for AppServerFn {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.0.build(
            "__app_server_fn_wrapper",
            quote! {
                #[allow(clippy::disallowed_macros)]
                #[allow(clippy::str_to_string)]
                #[allow(clippy::unsafe_derive_deserialize)]
            },
            quote! { #[leptos::server] },
        ));
    }
}
