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
use wax::{Glob, WalkEntry};

fn get_macro_invocation_file_path() -> String {
    let file_path = quote! { std::file!() };
    let file_path: TokenStream = file_path.into();

    syn::parse::<LitStr>(
        file_path.expand_expr().expect("file path should expand"),
    )
    .expect("file path should be a string literal")
    .value()
}

struct SelectorPath(String);

impl SelectorPath {
    fn new(macro_invocation_file_path: &str, el_id: Option<&str>) -> Self {
        let mut selector_path = macro_invocation_file_path.to_owned();
        selector_path.truncate(selector_path.len() - 3); // remove ".rs" from path

        if let Some(el_id) = el_id {
            selector_path.push('/');
            selector_path.push_str(el_id);
        }

        Self(selector_path)
    }

    fn to_hash(&self) -> u64 {
        RandomState::with_seeds(0, 0, 0, 0).hash_one(&self.0)
    }
}

impl syn::parse::Parse for SelectorPath {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let file_path = get_macro_invocation_file_path();
        let el_id = input.parse::<Ident>().ok().map(|el_id| el_id.to_string());
        Ok(Self::new(&file_path, el_id.as_deref()))
    }
}

impl ToTokens for SelectorPath {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let hash = self.to_hash();
        tokens.extend(quote! { #hash });
    }
}

#[proc_macro]
pub fn pin_test_selector(tokens: TokenStream) -> TokenStream {
    let selector = parse_macro_input!(tokens as SelectorPath);
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

fn get_macro_invocation_dir_path() -> String {
    let macro_invocation_file_path = get_macro_invocation_file_path();

    FILENAME_REG
        .replace(&macro_invocation_file_path, "")
        .to_string()
}

static COMPONENT_FILES_GLOB: LazyLock<Glob> = LazyLock::new(|| {
    Glob::new("**/{app,components,layouts,pages}/**/*.rs").unwrap()
});

static RS_FILES_GLOB: LazyLock<Glob> =
    LazyLock::new(|| Glob::new("**/*.rs").unwrap());

fn make_component_files_walk(
    dir_path: &str,
) -> impl Iterator<Item = WalkEntry<'static>> {
    // if macro is not invoked in a directory containing only components and
    // their tests, then we should limit the search to only directories that
    // may contain components
    let mut walk = COMPONENT_FILES_GLOB.walk(dir_path).peekable();

    if walk.peek().is_none() {
        walk = RS_FILES_GLOB.walk(dir_path).peekable();
    }

    walk.map(|v| v.expect("entry should be resolved"))
}

#[proc_macro]
pub fn generate_test_selectors(_tokens: TokenStream) -> TokenStream {
    let mut selector_ids = Vec::new();
    let mut selector_paths = Vec::new();

    let dir_path = get_macro_invocation_dir_path();

    for entry in make_component_files_walk(&dir_path) {
        let path = entry.path();
        let file = fs::File::open(path).unwrap();
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();

            if let Some(captures) = PIN_TEST_SELECTOR_REG.captures(&line) {
                let selector_path = SelectorPath::new(
                    &path.to_string_lossy(),
                    captures.name("el_id").map(|el_id| el_id.as_str()),
                );

                let id = selector_path.to_hash();
                selector_ids.push(id);

                let mut selector_path = selector_path
                    .0
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
