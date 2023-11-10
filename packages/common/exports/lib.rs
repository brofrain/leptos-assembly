pub use common_macros::{flatten_mod, flatten_pub_mod};
pub use derive_getters;
pub use derive_setters;
pub use educe;
pub use leptos;
pub use log;
pub use serde;
pub use time;

pub mod prelude {
    pub use common_macros::{flatten_mod, flatten_pub_mod};
    pub use derive_getters::Getters;
    pub use derive_setters::Setters;
    pub use educe::Educe;
    pub use leptos;
    pub use log;
    pub use serde::{self, Deserialize, Serialize};
}

#[cfg(feature = "client")]
pub mod client;

#[cfg(feature = "server")]
pub mod server;

#[macro_export]
macro_rules! use_macros {
    () => {
        #[macro_use]
        extern crate common_exports;
    };
}