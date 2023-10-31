use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};

use crate::app_item_fn_wrapper::AppItemFnWrapper;

pub struct AppComponent(AppItemFnWrapper);

impl Parse for AppComponent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(AppItemFnWrapper::parse(input)?))
    }
}

impl ToTokens for AppComponent {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.0.build(
            "__app_component_wrapper",
            quote! {
                #[allow(clippy::disallowed_macros)]
                #[allow(clippy::empty_structs_with_brackets)]
                #[allow(clippy::module_name_repetitions)]
            },
            quote! { #[leptos::component] },
        ));
    }
}
