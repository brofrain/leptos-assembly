use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};

use crate::item_fn_wrapper::ItemFnWrapper;

pub struct ServerFn(ItemFnWrapper);

impl Parse for ServerFn {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(ItemFnWrapper::parse(input)?))
    }
}

impl ToTokens for ServerFn {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.0.build(
            "__server_fn_wrapper",
            &quote! {
                #[allow(clippy::disallowed_macros)]
                #[allow(clippy::str_to_string)]
                #[allow(clippy::unsafe_derive_deserialize)]
            },
            &quote! { #[leptos::server] },
        ));
    }
}
