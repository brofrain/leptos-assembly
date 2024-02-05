common::use_macros!();

mod api {
    pub mod leptos_tag;
}
pub use api::*;

#[cfg(not(target_arch = "wasm32"))]
flatten_pub_mod!(app);
