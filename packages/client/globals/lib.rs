// @kw rename crate
// @kw i18n macros should not come from `client_macros`

pub mod prelude {
    pub use cfg_if::*;
    pub use client_macros::component;
    pub use common_macros::{flatten_mod, flatten_pub_mod};
    pub use derive_getters::*;
    pub use derive_more::*;
    pub use derive_setters::*;
    pub use educe::*;
    pub use leptos::{
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
    pub use serde::*;
    pub use time::ext::NumericalDuration;
    pub use unocss_classes::uno;
}

pub use prelude::*;
