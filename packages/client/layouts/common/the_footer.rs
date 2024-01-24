use client_components::{BaseIcon, BaseLink};
use client_composables::i18n;
use client_env::PROJECT_REPOSITORY_URL;
use client_i18n::{t_string, use_i18n};
use client_router::Route;
use exports::client::{icondata as i, prelude::*};
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
            <BaseLink title=Signal::derive(move || t_string!(i18n, nav.link_home)) to=Route::Home>
                <BaseIcon icon=i::AiHomeOutlined/>
            </BaseLink>

            <BaseLink
                title=Signal::derive(move || t_string!(i18n, nav.toggle_dark))
                on:click=toggle_dark
            >
                <BaseIcon icon=i::ChSun class="dark:hidden"/>
                <BaseIcon icon=i::TbMoon class="hidden dark:block"/>
            </BaseLink>

            <BaseLink
                title=Signal::derive(move || t_string!(i18n, nav.toggle_locale))
                on:click=toggle_locale
            >
                <BaseIcon icon=i::IoLanguage/>
            </BaseLink>

            <BaseLink title=Signal::derive(move || t_string!(i18n, nav.link_about)) to=Route::About>
                <BaseIcon icon=i::BsCardText/>
            </BaseLink>

            <BaseLink title="GitHub" to=PROJECT_REPOSITORY_URL>
                <BaseIcon icon=i::AiGithubFilled/>
            </BaseLink>
        </nav>
    }
}
