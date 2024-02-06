pub use common_macros as macros;
pub use common_macros::{flatten_mod, flatten_pub_mod};
pub use common_utils as utils;
pub use common_vendor as vendor;

#[macro_export]
macro_rules! use_macros {
    () => {
        #[macro_use]
        extern crate common;
    };
}

pub mod prelude {
    pub use common_macros::{flatten_mod, flatten_pub_mod};
    pub use common_vendor::{
        educe::Educe,
        getset::{CopyGetters, Getters, MutGetters, Setters},
        icondata as icon,
        leptos,
        leptos::{
            component,
            create_action,
            event_target_value,
            leptos_dom::*,
            on_cleanup,
            provide_context,
            server,
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
        },
        leptos_i18n::{self, t, t_string},
        log,
        num,
        serde::{self, de::DeserializeOwned, Deserialize, Serialize},
        time::ext::NumericalDuration,
        unocss_classes::{self, uno},
    };
}
