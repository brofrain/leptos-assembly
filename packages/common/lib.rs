pub use common_macros::{flatten_mod, flatten_pub_mod};

pub mod macros {
    #[cfg(feature = "client")]
    pub use common_macros::component;
    pub use common_macros::{
        cfg_csr,
        cfg_ssr,
        flatten_mod,
        flatten_pub_mod,
        is_csr,
        is_ssr,
        server, // @kw how to tackle backend?
    };
}

#[cfg(feature = "logger")]
pub mod logger {
    pub use common_logger::*;
}

#[cfg(feature = "utils")]
pub mod utils {
    pub use common_utils::*;
}

pub mod exports {
    cfg_if::cfg_if! {
        if #[cfg(feature = "client")] {
            pub use leptos;
            pub use leptos_i18n;
            pub use unocss_classes;
        }
    }

    pub use derive_getters;
    pub use derive_setters;
    pub use educe;
    pub use log;
    pub use serde;
    pub use time;
}

#[macro_export]
macro_rules! use_macros {
    () => {
        #[macro_use]
        extern crate common;
    };
}

pub mod prelude {
    #[cfg(feature = "client")]
    pub use exports::leptos::{
        self,
        create_action,
        event_target_value,
        leptos_dom::*,
        on_cleanup,
        provide_context,
        signal_prelude::*,
        spawn_local,
        update,
        use_context,
        view,
        with,
        Action,
        Callback,
        Children,
        DynAttrs,
        Effect,
        IntoView,
        Params,
        Resource,
        ServerFnError,
        Show,
        StoredValue,
        Suspense,
        View,
    };
    #[cfg(feature = "client")]
    pub use exports::leptos_i18n::{self, t};
    #[cfg(feature = "client")]
    pub use exports::unocss_classes::{self, uno};
    pub use exports::{
        derive_getters::Getters,
        derive_setters::Setters,
        educe::Educe,
        log,
        serde::{self, Deserialize, Serialize},
    };
    #[cfg(feature = "client")]
    pub use macros::component;
    pub use macros::{flatten_mod, flatten_pub_mod};

    use super::{exports, macros};
}
