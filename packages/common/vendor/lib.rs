pub use common_macros::{flatten_mod, flatten_pub_mod};
pub use educe;
pub use getset;
pub use leptos;
pub use log;
pub use num;
pub use serde;
pub use time;

pub mod prelude {
    pub use common_macros::{flatten_mod, flatten_pub_mod};
    pub use educe::Educe;
    pub use getset::{CopyGetters, Getters, MutGetters, Setters};
    pub use leptos;
    pub use log;
    pub use num;
    pub use serde::{self, de::DeserializeOwned, Deserialize, Serialize};
}

pub mod client;
pub mod server;
