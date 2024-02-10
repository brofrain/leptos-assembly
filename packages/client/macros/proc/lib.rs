#![feature(lazy_cell)]
#![feature(proc_macro_expand)]

use std::sync::{LazyLock, Mutex};

use ahash::{AHashMap, RandomState};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Ident, LitStr};

static SELECTOR_PATH_PER_ID: LazyLock<Mutex<AHashMap<u64, String>>> =
    LazyLock::new(|| Mutex::new(AHashMap::default()));

struct SelectorInfo {
    file_path: LitStr,
    el_id: Option<LitStr>,
}

impl syn::parse::Parse for SelectorInfo {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let file_path = quote! { std::file!() };
        let file_path: TokenStream = file_path.into();
        let file_path =
            syn::parse::<LitStr>(file_path.expand_expr().unwrap()).unwrap();

        let el_id = input
            .parse::<Ident>()
            .ok()
            .map(|el_id| LitStr::new(&el_id.to_string(), el_id.span()));

        Ok(Self { file_path, el_id })
    }
}

impl ToTokens for SelectorInfo {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut selector_path = self.file_path.value();
        selector_path.truncate(selector_path.len() - 3); // remove ".rs" from path

        if let Some(el_id) = &self.el_id {
            selector_path.push('/');
            selector_path.push_str(&el_id.value());
        }

        let id = RandomState::with_seeds(0, 0, 0, 0).hash_one(&selector_path);

        {
            let mut selector_path_per_id = SELECTOR_PATH_PER_ID.lock().unwrap();

            selector_path_per_id
                .entry(id)
                .and_modify(|_| {
                    let error_msg = LitStr::new(
                        &format!("Duplicate selector: {selector_path}"),
                        self.file_path.span(),
                    );

                    tokens.extend(quote! { compile_error!(#error_msg) });
                })
                .or_insert_with(|| {
                    tokens.extend(quote! { #id });
                    selector_path
                });
        }
    }
}

#[proc_macro]
pub fn register_test_selector(tokens: TokenStream) -> TokenStream {
    let selector = parse_macro_input!(tokens as SelectorInfo);
    TokenStream::from(selector.into_token_stream())
}

#[proc_macro]
pub fn generate_test_selectors_json(_tokens: TokenStream) -> TokenStream {
    TokenStream::new()
}
