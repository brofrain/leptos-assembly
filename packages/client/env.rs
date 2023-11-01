// @kw

use std::sync::LazyLock;

pub static CARGO_PKG_NAME: LazyLock<String> =
    LazyLock::new(|| env!("CARGO_PKG_NAME").replace('-', "_"));

pub const APP_REPOSITORY_URL: &str = env!("APP_REPOSITORY_URL");
