use macros::{__exports::cfg_if::cfg_if, cfg_csr, cfg_ssr};

cfg_if! {
    if #[cfg(any(feature = "csr", feature = "hydrate"))] {
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen]
        pub fn hydrate() {
            logger::init!();
            leptos::mount_to_body(client_app::App);
        }
    }
}

// @kw ugly code
// cfg_csr! {
//     use wasm_bindgen::prelude::*;

//     #[wasm_bindgen]
//     pub fn hydrate() {
//         logger::init!();
//         leptos::mount_to_body(client_app::App);
//     }
// }

cfg_ssr! {
    pub use client_app::App;
}
