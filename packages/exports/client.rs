pub use leptos_i18n;
pub use unocss_classes;

pub use super::*;

pub mod prelude {
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
    pub use leptos_i18n::{self, t};
    pub use macros::component;
    pub use time::ext::NumericalDuration;
    pub use unocss_classes::{self, uno};

    pub use crate::prelude::*;
}
