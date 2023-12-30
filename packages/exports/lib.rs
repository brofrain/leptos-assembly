pub use educe;
pub use getset;
pub use leptos;
pub use log;
pub use macros::{flatten_mod, flatten_pub_mod};
pub use num;
pub use serde;
pub use time;

pub mod prelude {
    pub use educe::Educe;
    pub use getset::{CopyGetters, Getters, MutGetters, Setters};
    pub use leptos;
    pub use log;
    pub use macros::{flatten_mod, flatten_pub_mod};
    pub use num;
    pub use serde::{self, de::DeserializeOwned, Deserialize, Serialize};
}

#[cfg(feature = "client")]
pub mod client;

#[cfg(feature = "server")]
pub mod server;

#[macro_export]
macro_rules! use_macros {
    () => {
        #[macro_use]
        extern crate exports;
    };
}
