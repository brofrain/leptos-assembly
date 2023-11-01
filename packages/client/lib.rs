#![feature(fn_traits)]
#![feature(unboxed_closures)]
#![feature(lazy_cell)]

#[macro_use]
extern crate common_macros;

#[macro_use]
extern crate client_macros;

mod env;
mod prelude;

mod app;
mod utils;

cfg_csr! {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn hydrate() {
        common_logger::init(env!("CARGO_CRATE_NAME"));
        leptos::mount_to_body(app::App);
    }
}

cfg_ssr! {
    pub use crate::app::App;
}
