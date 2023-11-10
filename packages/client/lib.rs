use common_macros::{cfg_csr, cfg_ssr};

cfg_csr! {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn hydrate() {
        common_logger::init!();
        leptos::mount_to_body(client_app::App);
    }
}

cfg_ssr! {
    pub use client_app::App;
}
