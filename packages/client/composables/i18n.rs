use client_globals::prelude::*;
use client_i18n::{t, use_i18n, Locale};
use client_utils::future::next_tick;

use super::toast;

pub fn use_toggle_locale_and_push_toast() -> impl Fn() {
    let i18n = use_i18n();

    move || {
        let current_locale = i18n.get_locale_untracked();

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

            toast::push(
                toast::Severity::Info,
                t!(i18n, locale.changed, new_locale),
            );
        });
    }
}
