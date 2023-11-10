use std::collections::HashSet;

use common_exports::client::prelude::*;
use leptos_use::storage::{
    use_storage_with_options,
    StorageType,
    UseStorageOptions,
};

use super::Store;

#[derive(Clone, Copy)]
pub struct Names {
    last_name: (Signal<Option<String>>, WriteSignal<Option<String>>),
    all: (Signal<HashSet<String>>, WriteSignal<HashSet<String>>),
    other_names_sorted: Memo<Vec<String>>,
}

fn storage_options<T>() -> UseStorageOptions<T> {
    UseStorageOptions::<T>::default()
        .storage_type(StorageType::Local)
        .debounce(1000.0)
}

impl Store for Names {
    fn create() -> Self {
        let (last_name, set_last_name, ..) = use_storage_with_options(
            "last-name",
            None::<String>,
            storage_options(),
        );
        let (names, set_names, ..) = use_storage_with_options(
            "names",
            HashSet::<String>::new(),
            storage_options(),
        );

        let other_names_sorted = Memo::new(move |_| {
            let mut names = names();

            if let Some(last_name) = &last_name() {
                names.remove(last_name);
            }

            let mut vec = Vec::from_iter(names);
            vec.sort_by_key(|v| v.to_lowercase());
            vec
        });

        Self {
            last_name: (last_name, set_last_name),
            all: (names, set_names),
            other_names_sorted,
        }
    }
}

impl Names {
    pub fn last_name(&self) -> Option<String> {
        (self.last_name.0)()
    }

    pub fn other_names_sorted(&self) -> Memo<Vec<String>> {
        self.other_names_sorted
    }

    pub fn push_name(&self, name: &str) {
        self.last_name.1.set(Some(name.to_owned()));
        self.all.1.update(|names| {
            names.insert(name.to_owned());
        });
    }
}
