exports::server::use_macros!();

flatten_pub_mod!(api);

#[cfg(not(target_arch = "wasm32"))]
flatten_pub_mod!(app);
