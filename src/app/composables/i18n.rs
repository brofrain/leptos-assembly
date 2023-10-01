use leptos_use::storage::{
    use_storage_with_options,
    StorageType,
    UseStorageOptions,
};

use super::toast;
use crate::{app::prelude::*, utils::future::next_tick};

leptos_i18n::load_locales!();
pub use i18n::*;

pub fn init() {
    let i18n = provide_i18n_context();

    if is_server!() {
        return;
    }

    let default_locale = i18n.get_locale_untracked();

    let (locale_storage, set_locale_storage, ..) = use_storage_with_options(
        "locale",
        default_locale,
        UseStorageOptions::default()
            .storage_type(StorageType::Local)
            .listen_to_storage_changes(false),
    );

    let stored_locale = locale_storage.get_untracked();

    if stored_locale != default_locale {
        i18n.set_locale(stored_locale);
    }

    _ = watch(
        i18n,
        move |v, _, _| {
            set_locale_storage(*v);
        },
        false,
    );
}

pub fn use_toggle_locale_and_push_toast() -> impl Fn() {
    let i18n = use_i18n();

    move || {
        let current_locale = i18n.get_locale();

        let new_locale = match current_locale {
            Locale::en => Locale::la,
            Locale::la => Locale::en,
        };

        i18n.set_locale(new_locale);

        spawn_local(async move {
            next_tick().await;

            let new_locale = match current_locale {
                Locale::en => "la",
                Locale::la => "en",
            };

            toast::push(toast::Payload {
                body: t_view_untracked!(i18n, locale.changed, new_locale),
                severity: toast::Severity::Info,
            });
        });
    }
}
