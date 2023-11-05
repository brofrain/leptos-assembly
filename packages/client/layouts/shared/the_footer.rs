use client_components::BaseLink;
use client_composables::i18n;
use client_env::PROJECT_REPOSITORY_URL;
use client_i18n::{t_string, use_i18n};
use client_router::Route;
use common::prelude::*;
use leptos_use::{use_color_mode, ColorMode, UseColorModeReturn};

#[component]
pub fn TheFooter() -> impl IntoView {
    let i18n = use_i18n();

    let UseColorModeReturn { mode, set_mode, .. } = use_color_mode();

    let toggle_dark = move |_| {
        set_mode(if mode() == ColorMode::Dark {
            ColorMode::Light
        } else {
            ColorMode::Dark
        });
    };

    let toggle_locale = {
        let toggle = i18n::use_toggle_locale_and_push_toast();
        move |_| toggle()
    };

    view! {
        <nav class=uno!["flex", "justify-center", "gap-3", "text-xl", "my-4"]>
            <BaseLink title=(move || t_string!(i18n, nav.link_home)).into_signal() to=Route::Home>

                <div class="icon-carbon-tree"></div>
            </BaseLink>

            <BaseLink
                title=(move || t_string!(i18n, nav.toggle_dark)).into_signal()
                on:click=toggle_dark
            >
                <div class="icon-carbon-sun dark:icon-carbon-moon"></div>
            </BaseLink>

            <BaseLink
                title=(move || t_string!(i18n, nav.toggle_locale)).into_signal()
                on:click=toggle_locale
            >
                <div class="icon-carbon-language"></div>
            </BaseLink>

            <BaseLink title=(move || t_string!(i18n, nav.link_about)).into_signal() to=Route::About>
                <div class="icon-carbon-dicom-overlay"></div>
            </BaseLink>

            <BaseLink title="GitHub" to=PROJECT_REPOSITORY_URL>
                <div class="icon-carbon-logo-github"></div>
            </BaseLink>
        </nav>
    }
}
