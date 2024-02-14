#![feature(lazy_cell)]
#![feature(proc_macro_expand)]

use std::{
    fs,
    io::{self, BufRead},
    sync::LazyLock,
};

use common::vendor::{
    ahash::{AHashSet, RandomState},
    serde_json,
};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use regex::Regex;
use syn::{parse_macro_input, Ident, LitStr};
use walkdir::WalkDir;

struct SelectorInfo {
    file_path: LitStr,
    el_id: Option<LitStr>,
}

impl syn::parse::Parse for SelectorInfo {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let file_path = quote! { std::file!() };
        let file_path: TokenStream = file_path.into();
        let file_path = syn::parse::<LitStr>(
            file_path.expand_expr().expect("file path should expand"),
        )
        .expect("file path should be a string literal");

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
        tokens.extend(quote! { #id });
    }
}

#[proc_macro]
pub fn pin_test_selector(tokens: TokenStream) -> TokenStream {
    let selector = parse_macro_input!(tokens as SelectorInfo);
    TokenStream::from(selector.into_token_stream())
}

fn generate_test_selectors_struct(
    prefix: &str,
    depth: usize,
    selector_ids: &[u64],
    selector_paths: &[Vec<String>],
) -> TokenStream2 {
    let mut child_structs = Vec::<TokenStream2>::new();
    let mut fields = Vec::<TokenStream2>::new();

    let mut field_names = AHashSet::<String>::new();

    for (i, selector_path_chunks) in selector_paths.iter().enumerate() {
        let selector_path_chunks_len = selector_path_chunks.len();

        if selector_path_chunks_len - 1 < depth {
            continue;
        }

        if selector_path_chunks_len - 1 == depth {
            let field_name = selector_path_chunks.last().unwrap();
            let field_name_ident =
                Ident::new(field_name, proc_macro2::Span::call_site());

            let id = selector_ids[i];

            fields.push(
                quote! { #[educe(Default = #id)] pub #field_name_ident: u64 },
            );
            continue;
        }

        let child_struct_name = &selector_path_chunks[depth];

        if field_names.contains(child_struct_name) {
            continue;
        }

        let prefixed_child_struct_name =
            format!("{prefix}_{child_struct_name}");

        let child_struct = generate_test_selectors_struct(
            &prefixed_child_struct_name,
            depth + 1,
            selector_ids,
            selector_paths,
        );

        child_structs.push(child_struct);

        let child_struct_name_ident =
            Ident::new(child_struct_name, proc_macro2::Span::call_site());

        let prefixed_child_struct_name_ident = Ident::new(
            &prefixed_child_struct_name,
            proc_macro2::Span::call_site(),
        );

        fields.push(
            quote! { pub #child_struct_name_ident: #prefixed_child_struct_name_ident },
        );

        field_names.insert(child_struct_name.clone());
    }

    let struct_ident = Ident::new(prefix, proc_macro2::Span::call_site());

    quote! {
        #(#child_structs)*

        #[derive(common::vendor::educe::Educe, Debug, Clone, Copy)]
        #[educe(Default)]
        pub struct #struct_ident {
            #(#fields),*
        }
    }
}

static FILENAME_REG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\/\w+\.rs$").unwrap());

static PIN_TEST_SELECTOR_REG: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"pin_test_selector!\(\s*(?P<el_id>\w*)\s*\)").unwrap()
});

#[proc_macro]
pub fn generate_test_selectors(_tokens: TokenStream) -> TokenStream {
    let mut selector_ids = Vec::new();
    let mut selector_paths = Vec::new();

    let file_path = quote! { std::file!() };
    let file_path: TokenStream = file_path.into();
    let file_path = syn::parse::<LitStr>(
        file_path.expand_expr().expect("file path should expand"),
    )
    .expect("file path should be a string literal")
    .value();

    let dir_path = FILENAME_REG.replace(&file_path, "").to_string();

    for entry in WalkDir::new(&dir_path) {
        let entry = entry.expect("directory entry should be resolved");

        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();

        let file = fs::File::open(path).unwrap();
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();

            if let Some(captures) = PIN_TEST_SELECTOR_REG.captures(&line) {
                let mut selector_path = path.to_string_lossy().to_string();

                selector_path.truncate(selector_path.len() - 3); // remove ".rs" from path

                if let Some(el_id) = captures.name("el_id") {
                    let el_id = el_id.as_str();

                    if !el_id.is_empty() {
                        selector_path.push('/');
                        selector_path.push_str(el_id);
                    }
                }

                let id = RandomState::with_seeds(0, 0, 0, 0)
                    .hash_one(&selector_path);

                selector_ids.push(id);

                let mut selector_path = selector_path
                    .strip_prefix(&dir_path)
                    .unwrap()
                    .split('/')
                    .map(str::to_string)
                    .collect::<Vec<_>>();

                selector_path.remove(0);

                selector_paths.push(selector_path);
            }
        }
    }

    let selectors = generate_test_selectors_struct(
        "selectors",
        0,
        &selector_ids,
        &selector_paths,
    );

    quote! {
        {
            mod __selectors {
                #selectors
            }

            __selectors::selectors::default()
        }
    }
    .into()
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
