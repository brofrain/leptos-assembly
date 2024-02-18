use std::{
    fs,
    io::{self, BufRead},
    sync::{Arc, LazyLock, RwLock},
};

use common::vendor::ahash::{AHashMap, AHashSet, RandomState};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use regex::Regex;
use syn::{Ident, LitStr};
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

pub struct SelectorPath(String);

impl SelectorPath {
    fn new(macro_invocation_file_path: &str, el_id: Option<&str>) -> Self {
        let mut selector_path = macro_invocation_file_path.to_owned();
        selector_path.truncate(selector_path.len() - 3); // remove ".rs" from path

        if let Some(el_id) = el_id
            && !el_id.is_empty()
        {
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

fn generate_test_selectors_struct(
    name: &str,
    depth: usize,
    selector_hashes: &[u64],
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

            let hash = selector_hashes[i];

            fields.push(quote! {
                #[educe(Default = concat!("[test='", #hash, "']"))]
                pub #field_name_ident: &'static str
            });
            continue;
        }

        let child_struct_name = &selector_path_chunks[depth];

        if field_names.contains(child_struct_name) {
            continue;
        }

        let prefixed_child_struct_name = format!("{name}_{child_struct_name}");

        let child_struct = generate_test_selectors_struct(
            &prefixed_child_struct_name,
            depth + 1,
            selector_hashes,
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

    let struct_ident = Ident::new(name, proc_macro2::Span::call_site());

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

    let mut dir_path = FILENAME_REG
        .replace(&macro_invocation_file_path, "")
        .to_string();
    dir_path.push('/');
    dir_path
}

fn make_component_files_walk(
    dir_path: &str,
) -> impl Iterator<Item = WalkEntry<'static>> {
    // if macro is not invoked in a directory containing only components and
    // their tests, then we should limit the search to only directories that
    // may contain components
    static COMPONENT_FILES_GLOB: LazyLock<Glob> = LazyLock::new(|| {
        Glob::new("**/{app,components,layouts,pages}/**/*.rs").unwrap()
    });

    static RS_FILES_GLOB: LazyLock<Glob> =
        LazyLock::new(|| Glob::new("**/*.rs").unwrap());

    let mut walk = COMPONENT_FILES_GLOB.walk(dir_path).peekable();

    if walk.peek().is_none() {
        walk = RS_FILES_GLOB.walk(dir_path).peekable();
    }

    walk.map(|v| v.expect("entry should be resolved"))
}

macro_rules! assert_ambiguous_selector_paths {
    ($cond:expr, $selector_path:expr, $other_selector_path:expr) => {
        assert!(
            $cond,
            "Ambiguous selector paths: `{}` and `{}`",
            $selector_path.join("/"),
            $other_selector_path.join("/")
        );
    };
}

fn check_for_ambiguous_selector_paths(selector_paths: &[Vec<String>]) {
    let selector_paths_len = selector_paths.len();
    for (i, selector_path) in selector_paths.iter().enumerate() {
        for other_selector_path in
            selector_paths.iter().take(selector_paths_len).skip(i + 1)
        {
            let len = selector_path.len();
            let other_len = other_selector_path.len();

            if len == other_len {
                assert_ambiguous_selector_paths!(
                    selector_path != other_selector_path,
                    selector_path,
                    other_selector_path
                );

                continue;
            }

            if len == other_len + 1 {
                assert_ambiguous_selector_paths!(
                    !selector_path.starts_with(other_selector_path),
                    selector_path,
                    other_selector_path
                );

                continue;
            }

            if len + 1 == other_len {
                assert_ambiguous_selector_paths!(
                    !other_selector_path.starts_with(selector_path),
                    selector_path,
                    other_selector_path
                );

                continue;
            }
        }
    }
}

pub fn generate() -> TokenStream {
    static CACHE: LazyLock<Arc<RwLock<AHashMap<String, String>>>> =
        LazyLock::new(|| Arc::new(RwLock::new(AHashMap::new())));

    let mut selector_hashes = Vec::new();
    let mut selector_paths = Vec::new();

    let dir_path = get_macro_invocation_dir_path();

    {
        let cache = CACHE.read().unwrap();
        if let Some(selectors) = cache.get(&dir_path) {
            return selectors.parse().unwrap();
        }
    }

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

                selector_hashes.push(selector_path.to_hash());

                let selector_path = selector_path
                    .0
                    .strip_prefix(&dir_path)
                    .unwrap()
                    .split('/')
                    .map(str::to_string)
                    .collect::<Vec<_>>();

                selector_paths.push(selector_path);
            }
        }
    }

    check_for_ambiguous_selector_paths(&selector_paths);

    let selectors = generate_test_selectors_struct(
        "selectors",
        0,
        &selector_hashes,
        &selector_paths,
    );

    let selectors = quote! {
        {
            mod __selectors {
                #selectors
            }

            __selectors::selectors::default()
        }
    };

    let selectors_string = selectors.to_string();
    CACHE.write().unwrap().insert(dir_path, selectors_string);

    selectors.into()
}
