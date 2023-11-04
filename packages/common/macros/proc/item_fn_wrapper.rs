use std::sync::atomic::{AtomicUsize, Ordering};

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_str,
    ItemFn,
    Visibility,
};

static LAST_WRAPPER_ID: AtomicUsize = AtomicUsize::new(0);

pub struct ItemFnWrapper {
    id: usize,
    vis: Visibility,
    item_fn: ItemFn,
}

impl Parse for ItemFnWrapper {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let id = LAST_WRAPPER_ID.load(Ordering::Acquire) + 1;
        LAST_WRAPPER_ID.store(id, Ordering::Release);

        let mut item_fn = input.parse::<ItemFn>()?;
        let vis = item_fn.vis;
        item_fn.vis = Visibility::Inherited;

        Ok(Self { id, vis, item_fn })
    }
}

impl ItemFnWrapper {
    pub fn build(
        &self,
        wrapper_module_name: &str,
        wrapper_module_prefix: &TokenStream,
        item_fn_prefix: &TokenStream,
    ) -> TokenStream {
        let ItemFnWrapper { id, vis, item_fn } = &self;

        let mut result = TokenStream::new();

        let module_name =
            parse_str::<Ident>(&format!("{wrapper_module_name}_{id}")).unwrap();

        result.extend(quote! {
            #wrapper_module_prefix
            mod #module_name {
                use super::*;
                #item_fn_prefix pub #item_fn
            }
        });

        result.extend(if let Visibility::Public(_) = vis {
            quote! { pub use #module_name::*; }
        } else {
            quote! { use #module_name::*; }
        });

        result
    }
}
