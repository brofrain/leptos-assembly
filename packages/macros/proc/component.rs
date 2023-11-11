use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};

use crate::item_fn_wrapper::ItemFnWrapper;

pub struct Component(ItemFnWrapper);

impl Parse for Component {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(ItemFnWrapper::parse(input)?))
    }
}

impl ToTokens for Component {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.0.build(
            "__component_wrapper",
            &quote! {
                #[allow(clippy::disallowed_macros)]
                #[allow(clippy::empty_structs_with_brackets)]
                #[allow(clippy::module_name_repetitions)]
            },
            &quote! { #[leptos::component] },
        ));
    }
}
