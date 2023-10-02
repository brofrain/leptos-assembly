use super::toast;
use crate::{app::prelude::*, utils::future::next_tick};

leptos_i18n::load_locales!();
pub use i18n::*;

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

            toast::push(toast::Payload {
                body: t_view_untracked!(i18n, locale.changed, new_locale),
                severity: toast::Severity::Info,
            });
        });
    }
}
