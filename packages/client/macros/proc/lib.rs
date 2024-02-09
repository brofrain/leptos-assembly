#![feature(lazy_cell)]
#![feature(proc_macro_expand)]

use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        LazyLock,
        Mutex,
    },
};

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Ident, LitStr};

#[derive(PartialEq, Eq, Hash)]
struct SelectorChunks(Vec<String>);

static LAST_SELECTOR_ID: AtomicUsize = AtomicUsize::new(0);

static SELECTOR_ID_PER_CHUNKS: LazyLock<Mutex<HashMap<SelectorChunks, usize>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

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
        let id = LAST_SELECTOR_ID.load(Ordering::Acquire) + 1;
        LAST_SELECTOR_ID.store(id, Ordering::Release);

        let mut file_path = self.file_path.value();
        file_path.truncate(file_path.len() - 3); // remove ".rs" from path
        let mut selector_chunks = file_path
            .split('/')
            .skip_while(|chunk| *chunk != "components" && *chunk != "pages")
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        if let Some(el_id) = &self.el_id {
            selector_chunks.push(el_id.value());
        }

        let selector_chunks = SelectorChunks(selector_chunks);

        {
            let mut id_per_selector = SELECTOR_ID_PER_CHUNKS.lock().unwrap();

            id_per_selector
                .entry(selector_chunks)
                .and_modify(|_| {
                    tokens.extend(
                        quote! { compile_error!("Duplicate selector") },
                    );
                })
                .or_insert_with(|| {
                    tokens.extend(quote! { #id });
                    id
                });
        }
    }
}

#[proc_macro]
pub fn inject_test_selector(tokens: TokenStream) -> TokenStream {
    let selector = parse_macro_input!(tokens as SelectorInfo);
    TokenStream::from(selector.into_token_stream())
}
