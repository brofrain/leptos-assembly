use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen]
        pub fn hydrate() {
            logger::init();
            leptos::mount_to_body(client_app::App);
        }
    }
}

pub use client_app::App;
